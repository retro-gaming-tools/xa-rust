//! Scripted input and screenshots, so a game can be driven with no display.
//!
//! `RECOMP_SCRIPT` holds steps separated by spaces: `wait:SECONDS`, `shot:NAME`
//! (written to `RECOMP_OUT`), `move:X,Y`, `click:X,Y`, `key:CODE` (a virtual
//! key), `type:TEXT` (characters, as typed), `fps` (the
//! frame rate since the last `fps`), `stick:X,Y` (the left stick, from -1 to 1),
//! `pad:south`, `pad:east` or `pad:start` (a controller button), `trigger:rt` or
//! `trigger:lt` (a trigger pressed and released), `peek:ADDRESS` (prints the
//! 32-bit value there, in hex), `poke:ADDRESS,VALUE` (writes one, both in hex), `answer:yes` or `answer:no` (how questions a
//! mod asks are answered, since there's no window to click) and `quit`.
//! For a game with a Windows menu bar: `command:ID` sends a menu command (in
//! hex), `menu` prints the menu as the game has it now, `pick:0,2` picks the
//! entry at those places in its lists, and `dialog:ID` prints a dialog
//! template (in hex).
//! With a display, as under `RECOMP_DISPLAY=1`, `stats` shows or hides the
//! frame counter, and `window:NAME` saves what the window shows, with the
//! counter or a mod's question over the picture, where `shot` saves the
//! game's frame alone.

use crate::cpu::Cpu;
use crate::host::{ddraw, gamepad};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WM_MOUSEMOVE: u32 = 0x200;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_CHAR: u32 = 0x102;
const MK_LBUTTON: u32 = 1;

pub struct Script {
    pub steps: VecDeque<String>,
    pub next_step: Instant,
    pub output: String,
    pub fps_mark: (u64, Instant),
    /// What `host::ask` answers with no window: yes unless `answer:no` said otherwise.
    pub answer: bool,
    /// Whether the run has a script at all.
    pub scripted: bool,
}

impl Script {
    pub fn from_environment() -> Script {
        let text = std::env::var("RECOMP_SCRIPT").unwrap_or_default();
        Script {
            steps: text.split_whitespace().map(str::to_string).collect(),
            next_step: Instant::now(),
            output: std::env::var("RECOMP_OUT").unwrap_or_else(|_| ".".to_string()),
            fps_mark: (0, Instant::now()),
            answer: true,
            scripted: !text.trim().is_empty(),
        }
    }
}

fn pair(value: &str) -> (u32, u32) {
    let (x, y) = value.split_once(',').unwrap_or((value, "0"));
    (x.parse().unwrap_or(0), y.parse().unwrap_or(0))
}

fn print_menu(items: &[crate::host::video::menu::Item], depth: usize) {
    for (place, item) in items.iter().enumerate() {
        let state = format!("{}{}", if item.disabled { " greyed" } else { "" }, if item.checked { " ticked" } else { "" });
        let what = if item.separator { "line".to_string() } else { format!("{:?} {:x}", item.text, item.command) };
        println!("menu {}{place} {what}{state}", "  ".repeat(depth));
        print_menu(&item.children, depth + 1);
    }
}

/// How many different colours an RGB picture has, counting up to 256.
pub(crate) fn colours(pixels: &[u8]) -> usize {
    let mut seen = std::collections::HashSet::new();
    for pixel in pixels.chunks_exact(3) {
        seen.insert((pixel[0], pixel[1], pixel[2]));
        if seen.len() >= 256 {
            break;
        }
    }
    seen.len()
}

thread_local! {
    /// Every `shot` of the run by name, for `changed:` to compare with.
    static SHOTS: std::cell::RefCell<std::collections::HashMap<String, (u32, u32, Vec<u8>)>> = Default::default();
}

/// The picture on screen as RGB: Glide's or DirectDraw's, or a DOS program's
/// VGA or VESA picture when it isn't using Glide.
fn picture_now(cpu: &mut Cpu) -> Option<(u32, u32, Vec<u8>)> {
    ddraw::current_frame(cpu).or_else(|| {
        let picture = cpu.host.timers.dos.then(|| super::vga::frame(cpu)).flatten()?;
        let bytes = picture.iter().flat_map(|p| [((p >> 11) as u32 * 255 / 31) as u8, ((p >> 5 & 63) as u32 * 255 / 63) as u8, ((p & 31) as u32 * 255 / 31) as u8]);
        Some((super::GAME_WIDTH as u32, super::GAME_HEIGHT as u32, bytes.collect()))
    })
}

fn remember(name: &str, width: u32, height: u32, pixels: &[u8]) {
    SHOTS.with(|shots| shots.borrow_mut().insert(name.to_string(), (width, height, pixels.to_vec())));
}

/// The share of pixels, from 0 to 1, that differ between two RGB pictures.
/// Pictures of different sizes differ entirely.
pub(crate) fn changed_share(a: (u32, u32, &[u8]), b: (u32, u32, &[u8])) -> f64 {
    if a.0 != b.0 || a.1 != b.1 || a.2.len() != b.2.len() || a.2.is_empty() {
        return 1.0;
    }
    let differing = a.2.chunks_exact(3).zip(b.2.chunks_exact(3)).filter(|(x, y)| x != y).count();
    differing as f64 / (a.2.len() / 3) as f64
}

/// The brightness of each pixel in an area given in game pixels (640x480),
/// from 0 to 255, from an RGB picture of any size.
pub(crate) fn area_brightness((width, height, pixels): (u32, u32, &[u8]), [x, y, w, h]: [u32; 4]) -> Vec<f64> {
    let (sx, sy) = (width as f64 / 640.0, height as f64 / 480.0);
    let mut values = Vec::new();
    for gy in y..(y + h).min(480) {
        for gx in x..(x + w).min(640) {
            let (px, py) = (((gx as f64 + 0.5) * sx) as u32, ((gy as f64 + 0.5) * sy) as u32);
            let at = ((py.min(height - 1) * width + px.min(width - 1)) * 3) as usize;
            let (r, g, b) = (pixels[at] as f64, pixels[at + 1] as f64, pixels[at + 2] as f64);
            values.push(0.299 * r + 0.587 * g + 0.114 * b);
        }
    }
    values
}

/// The mean and standard deviation of some values.
pub(crate) fn mean_and_spread(values: &[f64]) -> (f64, f64) {
    let count = values.len().max(1) as f64;
    let mean = values.iter().sum::<f64>() / count;
    let spread = (values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / count).sqrt();
    (mean, spread)
}

/// How much of a picture looks like TV static, from 0 to 1: the share of
/// neighbouring game pixels in the middle of the screen whose brightness
/// jumps by more than 60. A 3D scene or a menu stays far below a tenth;
/// Extreme Assault's static, in 2x2 blocks, measures about 0.3.
pub(crate) fn static_share(picture: (u32, u32, &[u8])) -> f64 {
    let values = area_brightness(picture, [120, 120, 400, 240]);
    let jumps = values.windows(2).filter(|pair| (pair[0] - pair[1]).abs() > 60.0).count();
    jumps as f64 / values.len().max(2) as f64
}

#[cfg(test)]
mod tests {
    #[test]
    fn area_measures() {
        // A 4x2 picture scaled to the game's 640x480: left half black, right white.
        let pixels: Vec<u8> = (0..8).flat_map(|i| if i % 4 < 2 { [0u8; 3] } else { [255u8; 3] }).collect();
        let values = super::area_brightness((4, 2, &pixels), [0, 0, 640, 480]);
        let (mean, spread) = super::mean_and_spread(&values);
        assert!((mean - 127.5).abs() < 0.5 && (spread - 127.5).abs() < 0.5);
        assert_eq!(super::mean_and_spread(&super::area_brightness((4, 2, &pixels), [0, 0, 320, 480])), (0.0, 0.0));
        // Noise: alternating pixels at the game's size.
        let noise: Vec<u8> = (0..640 * 480).flat_map(|i| if (i + i / 640) % 2 == 0 { [0u8; 3] } else { [255u8; 3] }).collect();
        assert!(super::static_share((640, 480, &noise)) > 0.9);
        assert!(super::static_share((4, 2, &pixels)) < 0.01);
    }

    #[test]
    fn changed_share_counts_differing_pixels() {
        let a = [0u8; 12];
        let mut b = a;
        b[3] = 9;
        assert_eq!(super::changed_share((2, 2, &a), (2, 2, &a)), 0.0);
        assert_eq!(super::changed_share((2, 2, &a), (2, 2, &b)), 0.25);
        assert_eq!(super::changed_share((2, 2, &a), (4, 1, &b)), 1.0);
    }

    #[test]
    fn colours_counts_distinct_pixels() {
        assert_eq!(super::colours(&[0; 30]), 1);
        assert_eq!(super::colours(&[0, 0, 0, 1, 0, 0, 0, 0, 0]), 2);
        let many: Vec<u8> = (0..1000u32).flat_map(|i| [i as u8, (i >> 8) as u8, 0]).collect();
        assert_eq!(super::colours(&many), 256);
    }
}

/// Writes a PNG with stored blocks: it needs no compression library.
pub(crate) fn write_png(path: &str, width: u32, height: u32, pixels: &[u8]) {
    fn crc(data: &[u8]) -> u32 {
        let mut value = 0xffff_ffffu32;
        for &byte in data {
            value ^= byte as u32;
            for _ in 0..8 {
                value = if value & 1 != 0 { (value >> 1) ^ 0xedb8_8320 } else { value >> 1 };
            }
        }
        !value
    }
    fn chunk(file: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        file.extend((data.len() as u32).to_be_bytes());
        let start = file.len();
        file.extend(kind);
        file.extend(data);
        let check = crc(&file[start..]);
        file.extend(check.to_be_bytes());
    }
    let mut raw = Vec::new();
    for row in pixels.chunks((width * 3) as usize) {
        raw.push(0);
        raw.extend(row);
    }
    let mut stream = vec![0x78, 1];
    let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (index, block) in blocks.iter().enumerate() {
        stream.push(u8::from(index + 1 == blocks.len()));
        stream.extend((block.len() as u16).to_le_bytes());
        stream.extend((!(block.len() as u16)).to_le_bytes());
        stream.extend(*block);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in &raw {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    stream.extend(((b << 16) | a).to_be_bytes());
    let mut file = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut header = Vec::new();
    header.extend(width.to_be_bytes());
    header.extend(height.to_be_bytes());
    header.extend([8, 2, 0, 0, 0]);
    chunk(&mut file, b"IHDR", &header);
    chunk(&mut file, b"IDAT", &stream);
    chunk(&mut file, b"IEND", &[]);
    if let Err(error) = std::fs::write(path, file) {
        println!("couldn't write {path}: {error}");
    }
}

/// Runs every step that's due. Called each time the game looks for messages.
pub fn advance(cpu: &mut Cpu) {
    while Instant::now() >= cpu.host.script.next_step {
        let Some(step) = cpu.host.script.steps.pop_front() else { return };
        let (name, value) = step.split_once(':').unwrap_or((&step, ""));
        match name {
            "wait" => {
                let seconds: f64 = value.parse().unwrap_or(0.0);
                cpu.host.script.next_step = Instant::now() + Duration::from_secs_f64(seconds);
            }
            // `look:NAME`: keeps the picture for later checks, as `shot` does,
            // without writing a file, for bursts.
            "look" => {
                if let Some((width, height, pixels)) = picture_now(cpu) {
                    remember(value, width, height, &pixels);
                }
            }
            "shot" => match picture_now(cpu) {
                Some((width, height, pixels)) => {
                    let path = format!("{}/{value}.png", cpu.host.script.output);
                    write_png(&path, width, height, &pixels);
                    // Checks fail a black or flat frame by this count.
                    println!("frame {} saved as {value}.png, {} colours", cpu.host.draw.frames, colours(&pixels));
                    remember(value, width, height, &pixels);
                }
                None => println!("no frame yet for {value}"),
            },
            "window" => match cpu.host.video.as_mut().and_then(|video| video.window_picture()) {
                Some((width, height, pixels)) => {
                    let path = format!("{}/{value}.png", cpu.host.script.output);
                    write_png(&path, width, height, &pixels);
                    println!("window saved as {value}.png");
                }
                None => println!("no window drawn with OpenGL for {value}"),
            },
            "stats" => crate::host::video::toggle_stats(cpu),
            "move" | "click" => {
                let (x, y) = pair(value);
                cpu.host.pointer = (x, y);
                // A DOS program that moves its own pointer by the mouse's
                // movement follows ours exactly once both sit in an edge,
                // so a move into an edge pushes on past it.
                if cpu.host.timers.dos {
                    const PAST: i32 = 1000;
                    let push = |at: u32, last: u32| if at == 0 { -PAST } else if at >= last { PAST } else { 0 };
                    crate::host::dos::push_past_edge(cpu, push(x, 639), push(y, 479));
                }
                let position = x | (y << 16);
                cpu.host.post((WM_MOUSEMOVE, 0, position));
                if name == "click" {
                    cpu.host.post((WM_LBUTTONDOWN, MK_LBUTTON, position));
                    cpu.host.post((WM_LBUTTONUP, 0, position));
                    // A DOS program's mouse handler hears the left button go
                    // down and up.
                    crate::host::dos::mouse_button_state(cpu, 1);
                    crate::host::dos::mouse_button_state(cpu, 0);
                }
            }
            "key" if cpu.host.timers.dos => {
                // A DOS program reads scan codes from its keyboard interrupt.
                let code = value.parse().unwrap_or(0);
                if let Some(scancode) = crate::host::dinput::code_of_virtual_key(code) {
                    crate::host::pc::key(cpu, scancode, true);
                    crate::host::pc::key(cpu, scancode, false);
                }
            }
            "key" => {
                let code = value.parse().unwrap_or(0);
                cpu.host.post((WM_KEYDOWN, code, 0));
                // As the keyboard does: these keys are characters as well.
                if matches!(code, 8 | 9 | 13 | 27) {
                    cpu.host.post((WM_CHAR, code, 0));
                }
                cpu.host.post((WM_KEYUP, code, 0));
            }
            "type" => {
                for character in value.chars().filter(char::is_ascii) {
                    let code = character.to_ascii_uppercase() as u32;
                    cpu.host.post((WM_KEYDOWN, code, 0));
                    cpu.host.post((WM_CHAR, character as u32, 0));
                    cpu.host.post((WM_KEYUP, code, 0));
                }
            }
            "stick" => {
                let (x, y) = value.split_once(',').unwrap_or((value, "0"));
                gamepad::set_axis(cpu, 0, x.parse().unwrap_or(0.0));
                gamepad::set_axis(cpu, 1, y.parse().unwrap_or(0.0));
            }
            "peek" => {
                let at = u32::from_str_radix(value, 16).unwrap_or(0);
                println!("peek {at:x} = {:#x}", cpu.r32(at));
            }
            // `expect:ADDRESS,VALUE[,MASK]`, all in hex: ends the run with an
            // error when the 32-bit value there, masked, differs, so a check
            // can prove the game reached the level or loaded the save it should.
            "expect" => {
                let mut parts = value.split(',').map(|part| u32::from_str_radix(part, 16).unwrap_or(0));
                let (at, wanted, mask) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(u32::MAX));
                let found = cpu.r32(at) & mask;
                if found != wanted & mask {
                    println!("expect failed: {at:x} holds {found:#x}, not {wanted:#x}");
                    std::process::exit(3);
                }
                println!("expect {at:x} = {found:#x}: ok");
            }
            // `changed:NAME,PERCENT`: ends the run with an error unless at
            // least PERCENT of the pixels differ from the `shot:NAME` taken
            // earlier, so an animated menu doesn't pass for a new level.
            "changed" => {
                let (name, percent) = value.split_once(',').unwrap_or((value, "20"));
                let percent: f64 = percent.parse().unwrap_or(20.0);
                let earlier = SHOTS.with(|shots| shots.borrow().get(name).cloned());
                let share = match (earlier, picture_now(cpu)) {
                    (Some((w, h, old)), Some((width, height, now))) => changed_share((w, h, &old), (width, height, &now)) * 100.0,
                    _ => 0.0,
                };
                if share < percent {
                    println!("expect failed: only {share:.1}% of the screen changed from {name}, not {percent}%");
                    std::process::exit(3);
                }
                println!("changed {share:.1}% from {name}: ok");
            }
            // `varied:X,Y,W,H,MIN` and `bright:X,Y,W,H,MIN`, in game pixels:
            // end the run with an error unless the area's brightness spreads
            // by at least MIN (a see-through box or HUD panel, not a solid
            // one), or averages at least MIN (no dark band where the 3D shows).
            "varied" | "bright" => {
                let numbers: Vec<f64> = value.split(',').map(|part| part.trim().parse().unwrap_or(0.0)).collect();
                let area = [0, 1, 2, 3].map(|index| numbers.get(index).copied().unwrap_or(0.0) as u32);
                let least = numbers.get(4).copied().unwrap_or(0.0);
                let measured = picture_now(cpu).map(|(width, height, pixels)| {
                    let (mean, spread) = mean_and_spread(&area_brightness((width, height, &pixels), area));
                    if name == "varied" { spread } else { mean }
                });
                let measured = measured.unwrap_or(0.0);
                if measured < least {
                    println!("expect failed: {name} {value} measured {measured:.1}");
                    std::process::exit(3);
                }
                println!("{name} {value} measured {measured:.1}: ok");
            }
            // `blended:MIN`: ends the run with an error unless Glide's last
            // frame blended at least MIN pixels of 2D through the pixel
            // pipeline (Extreme Assault's shadows, see-through boxes, radar
            // and HUD), whatever the scene behind them.
            "blended" => {
                let least: u64 = value.trim().parse().unwrap_or(1);
                let found = super::glide::blended_last_frame(cpu);
                if found < least {
                    println!("expect failed: the last frame blended {found} pixels of 2D, not {least}");
                    std::process::exit(3);
                }
                println!("blended {found} pixels: ok");
            }
            // `copied:MAX`: ends the run with an error when Glide's last frame
            // copied more than MAX pixels of 2D. Extreme Assault's HUD is a
            // small part of the screen; a 640x480 copy of the 3D drawn back
            // over the scene (301,235 pixels) made missions look 640x480.
            "copied" => {
                let most: u64 = value.trim().parse().unwrap_or(u64::MAX);
                let found = super::glide::copied_last_frame(cpu);
                if found > most {
                    println!("expect failed: the last frame copied {found} pixels of 2D, more than {most}");
                    std::process::exit(3);
                }
                println!("copied {found} pixels: ok");
            }
            // `noisy:PREFIX`: ends the run with an error unless one of the
            // shots whose names start with PREFIX shows TV static, as Extreme
            // Assault's briefings do at each cut. A burst of shots catches it.
            "noisy" => {
                let best = SHOTS.with(|shots| {
                    shots.borrow().iter().filter(|(shot, _)| shot.starts_with(value)).map(|(_, (w, h, pixels))| static_share((*w, *h, pixels))).fold(0.0, f64::max)
                });
                // Extreme Assault's static comes in 2x2 blocks, so at most
                // every second pair of neighbours jumps: it measures about 30%.
                if best < 0.15 {
                    println!("expect failed: no shot {value}* shows static (most {:.0}%)", best * 100.0);
                    std::process::exit(3);
                }
                println!("noisy {value}: {:.0}% static: ok", best * 100.0);
            }
            // Writes a 32-bit value, both in hex, such as a level number, so a
            // check can start where the game would otherwise take hours to reach.
            "poke" => {
                let (at, wanted) = value.split_once(',').unwrap_or((value, "0"));
                let (at, wanted) = (u32::from_str_radix(at, 16).unwrap_or(0), u32::from_str_radix(wanted, 16).unwrap_or(0));
                cpu.w32(at, wanted);
                println!("poke {at:x} = {wanted:#x}");
            }
            // Holds a controller axis, numbered as SDL does (0 and 1 the left
            // stick's x and y, 2 and 3 the right's), at a value from -1 to 1,
            // until another `stick` step moves it.
            "stick" => {
                let (axis, amount) = value.split_once(',').unwrap_or((value, "0"));
                let (axis, amount) = (axis.parse().unwrap_or(0), amount.parse().unwrap_or(0.0));
                println!("stick {axis} at {amount}");
                gamepad::set_axis(cpu, axis, amount);
            }
            // Holds or lets go of a controller button, by SDL's number (0 is
            // south, 1 east, 2 west, 3 north): `hold:0,1` and `hold:0,0`.
            "hold" => {
                let (button, down) = value.split_once(',').unwrap_or((value, "1"));
                gamepad::set_button(cpu, button.parse().unwrap_or(0), down != "0");
            }
            "trigger" => {
                let axis = if value == "lt" { gamepad::AXIS_LEFT_TRIGGER } else { gamepad::AXIS_RIGHT_TRIGGER };
                println!("trigger {value} at {:?}", cpu.host.pointer);
                gamepad::set_axis(cpu, axis, 1.0);
                gamepad::set_axis(cpu, axis, 0.0);
            }
            "pad" => {
                let button = match value {
                    "east" => gamepad::BUTTON_EAST,
                    "start" => gamepad::BUTTON_START,
                    _ => gamepad::BUTTON_SOUTH,
                };
                println!("pad {value} at {:?}", cpu.host.pointer);
                gamepad::set_button(cpu, button, true);
                gamepad::set_button(cpu, button, false);
            }
            "audio" => {
                println!("sound peak {:.3}, music peak {:.3}", cpu.host.audio.peak, cpu.host.audio.peak_music);
                cpu.host.audio.peak = 0.0;
                cpu.host.audio.peak_music = 0.0;
            }
            "fps" => {
                let frames = cpu.host.draw.frames + cpu.host.glide.as_ref().map_or(0, |glide| glide.frames);
                let (last_frames, last_time) = std::mem::replace(&mut cpu.host.script.fps_mark, (frames, Instant::now()));
                let seconds = last_time.elapsed().as_secs_f64();
                println!("{:.1} frames a second", (frames - last_frames) as f64 / seconds);
                if let Some(glide) = cpu.host.glide.as_mut() {
                    let [drawing, flat, locks, scans, scan_count] = std::mem::take(&mut glide.spent);
                    println!("{:.0}% scanning 2D at {:.1} mode changes a frame", scans / seconds * 100.0, scan_count / (frames - last_frames).max(1) as f64);
                    println!("{:.0}% of the time drawing, {:.0}% on 2D, {:.1} 2D locks a frame", drawing / seconds * 100.0, flat / seconds * 100.0, locks / (frames - last_frames).max(1) as f64);
                    println!("longest frame {:.0} ms", std::mem::take(&mut glide.longest_frame) * 1000.0);
                }
                let waited = std::mem::take(&mut cpu.host.waited);
                let presented = std::mem::take(&mut cpu.host.presented);
                println!("{:.0}% asleep, {:.0}% putting frames on screen", waited / seconds * 100.0, presented / seconds * 100.0);
                let (reads, retrace) = std::mem::take(&mut cpu.host.retrace);
                println!("{reads} VGA status reads, {:.0}% asleep in them", retrace / seconds * 100.0);
                // How evenly frames reached the screen, which an average hides.
                let mut shown = std::mem::take(&mut cpu.host.shown_frames);
                if !shown.is_empty() {
                    shown.sort_by(f32::total_cmp);
                    let at = |share: f32| shown[((shown.len() - 1) as f32 * share) as usize];
                    println!(
                        "frames shown {:.1} ms apart in the middle, 10% to 90% {:.1} to {:.1}, 1% {:.1}, longest {:.1}",
                        at(0.5), at(0.1), at(0.9), at(0.99), shown[shown.len() - 1]
                    );
                }
            }
            "profile" => {
                cpu.hits.clear();
                cpu.profiling = true;
            }
            "hits" => {
                cpu.profiling = false;
                let low: u32 = u32::from_str_radix(value, 16).unwrap_or(0);
                let mut hits: Vec<(u32, u64)> = cpu.hits.iter().map(|(a, n)| (*a, *n)).filter(|(a, _)| *a >= low && *a < low + 0x0100_0000).collect();
                hits.sort();
                for (address, count) in hits {
                    println!("hit {:06x} {count}", address - low + 0x40_0000);
                }
            }
            // A menu command, by its id in hexadecimal, as picking it from a
            // Windows menu bar sends it.
            "command" => {
                const WM_COMMAND: u32 = 0x111;
                let id = u32::from_str_radix(value, 16).unwrap_or(0);
                cpu.host.messages.push_back((WM_COMMAND, id, 0));
            }
            // The game's menu as the panel would list it, with what the game
            // has greyed out or ticked.
            "menu" => match cpu.host.menus.get(&cpu.host.menu_shown) {
                Some(items) => print_menu(items, 0),
                None => println!("the window has no menu"),
            },
            // The values on the guest stack that point into the program's code:
            // a rough call stack of where the game is waiting for messages.
            "stack" => {
                let base = cpu.host.modules.first().map_or(0x40_0000, |module| module.base);
                let found: Vec<String> = (0..256u32)
                    .map(|index| cpu.r32(cpu.esp.wrapping_add(index * 4)))
                    .filter(|value| (base + 0x1000..base + 0x9_e400).contains(value))
                    .map(|value| format!("{value:x}"))
                    .collect();
                println!("stack {}", found.join(" "));
            }
            // A tap of Alt, which opens the game's menu anywhere.
            "alt" => cpu.host.menu_wanted = true,
            // The menu's own last row, Quit, which ends the program.
            "menu-quit" => {
                println!("the player quit from the menu");
                std::process::exit(0);
            }
            // Records the next SECONDS of sound, as mixed, to NAME.wav.
            "record" => {
                let (name, seconds) = value.split_once(',').unwrap_or((value, "10"));
                let path = format!("{}/{name}.wav", cpu.host.script.output);
                crate::host::audio::record(cpu, path, seconds.parse().unwrap_or(10.0));
            }
            // A dialog template of the program, by its number in hexadecimal.
            "dialog" => crate::host::dialog_box::print(cpu, u32::from_str_radix(value, 16).unwrap_or(0)),
            // For the dialog that is open: `press:ID` presses a control,
            // `slide:ID,VALUE` moves a slider and `line:ID,LINE` selects a
            // line of a list. Ids are in hexadecimal.
            "press" | "slide" | "line" => {
                use crate::host::dialog_box::{ask, Asked};
                let (id, number) = value.split_once(',').unwrap_or((value, "0"));
                let id = u32::from_str_radix(id, 16).unwrap_or(0);
                let asked = match name {
                    "press" => Asked::Press(id),
                    "slide" => Asked::Slide(id, number.parse().unwrap_or(0)),
                    _ => Asked::Line(id, number.parse().unwrap_or(0)),
                };
                if !ask(cpu, asked) {
                    println!("no dialog is open for {step}");
                }
            }
            // Picks an entry of the menu by its place in each list, counted
            // from 0, such as `pick:0,0`, as the panel does for a player.
            "pick" => {
                const WM_COMMAND: u32 = 0x111;
                let mut list = cpu.host.menus.get(&cpu.host.menu_shown).map_or(&[][..], |items| &items[..]);
                let mut picked = None;
                for place in value.split(',') {
                    picked = place.parse().ok().and_then(|place: usize| list.get(place));
                    list = picked.map_or(&[][..], |item| &item.children[..]);
                }
                match picked.filter(|item| item.children.is_empty() && !item.separator && !item.disabled) {
                    Some(item) => {
                        println!("picked {} ({:x})", item.text, item.command);
                        let command = item.command;
                        cpu.host.messages.push_back((WM_COMMAND, command, 0));
                    }
                    None => println!("nothing to pick at {value}"),
                }
            }
            "answer" => cpu.host.script.answer = value != "no",
            "quit" => {
                println!("the script ended the run");
                std::process::exit(0);
            }
            _ => println!("unknown script step: {step}"),
        }
    }
}
