//! DOS, the BIOS and the DOS/4GW extender, for recompiled DOS programs.
//!
//! A flat 32-bit DOS program has no imports: it asks for files, memory, the
//! screen, the mouse and the clock through software interrupts, `int 21h` for
//! DOS, `int 31h` for the extender's DPMI services, `int 10h` for the video
//! BIOS and so on. Each `int` in the translated code calls `interrupt` here,
//! with the registers as the program set them.
//!
//! Memory is laid out as DOS/4GW gave it: the first megabyte is DOS's, with
//! the BIOS data at 0x400, the program's PSP and environment, and the VGA
//! memory at 0xA0000, and the program itself is loaded higher up (see `le`).
//! Its data selector is flat, based at 0. The PSP and the environment have
//! selectors of their own, as under DOS/4GW, which the program's start-up
//! code reads them through.
//!
//! Only what a game reaches is answered. Anything else stops with a named
//! error, so gaps show up as they're met, as missing imports do for Windows.

use crate::cpu::Cpu;
use std::collections::HashMap;

/// The flat selector of code, data and stack, based at 0.
pub const FLAT: u16 = 0x2b;
/// DOS/4GW's selector for the PSP, which Watcom's start-up code expects.
const PSP_SELECTOR: u16 = 0x24;
const ENVIRONMENT_SELECTOR: u16 = 0x2c;
/// Where the PSP and the environment are, in the first megabyte.
const PSP: u32 = 0x5000;
const ENVIRONMENT: u32 = 0x5200;
const ES: usize = 0;
const CS: usize = 1;
const SS: usize = 2;
const DS: usize = 3;
const FS: usize = 4;
const GS: usize = 5;

#[derive(Default)]
pub struct Dos {
    /// Selectors the program can load, and their bases.
    selectors: HashMap<u16, u32>,
    /// DOS file handles, from 5 up, and the runtime's handles for them.
    handles: HashMap<u16, u32>,
    /// Where `find first` writes what it found (DOS's disk transfer area).
    transfer_area: u32,
    /// What `find first` found and `find next` hands out.
    found: Vec<(String, bool, u64)>,
    /// Interrupt handlers the program set: selector and offset.
    pub vectors: HashMap<u8, (u16, u32)>,
    /// Where the next DOS memory block goes.
    dos_next: u32,
    /// The handler every vector has before the program sets its own: a
    /// runtime function that returns as `iretd` does (see `previous_handler`).
    previous: u32,
    /// The pointer's place at the mouse driver's last movement report.
    mouse_seen: (u32, u32),
    /// The current folder, from the game folder's root, with `\` between
    /// its parts and none at either end: empty for the root. Extreme Assault
    /// changes into `DATA.SFX` and opens `SOUND\SAMPLES.SET` from there.
    folder: String,
    /// The program's mouse handler from `int 33h` AX=0Ch, and the events it
    /// wants: Extreme Assault takes all its pointer movement and clicks this
    /// way, and never asks for the position.
    mouse_handler: Option<(u32, u32)>,
    /// The pointer and buttons as last told to that handler.
    mouse_told: (u32, u32, u32),
    /// The driver's running counts of movement, in mickeys, one a game
    /// pixel. The handler gets them in SI and DI and works out the movement
    /// itself from the counts it saw last.
    mickeys: (u32, u32),
    /// Movement still to tell, beyond what the pointer shows: a scripted
    /// move into an edge pushes on past it, as a mouse can.
    mickeys_beyond: (i32, i32),
    /// The handler has had its first call, which Extreme Assault's takes as
    /// the starting counts and not as movement.
    mouse_started: bool,
    /// Button states not yet told, oldest first, so a click that goes down
    /// and up between two safe points still reaches the handler as both.
    mouse_buttons: std::collections::VecDeque<u32>,
    /// When the handler last heard a queued button state. The next waits a
    /// while: the software `XA.EXE` looks at the buttons once a frame, and
    /// missed a press and release told one right after the other.
    mouse_button_told: Option<std::time::Instant>,
}

/// How long a queued button state is held before the next is told, about as
/// long as a quick click by hand.
const BUTTON_HELD: std::time::Duration = std::time::Duration::from_millis(60);

/// Movement past where the pointer can go, for a scripted move into an
/// edge: the program's own pointer, which may have started elsewhere, then
/// ends up in that edge too, so the two match from there on.
pub fn push_past_edge(cpu: &mut Cpu, x: i32, y: i32) {
    let beyond = &mut cpu.host.dos.mickeys_beyond;
    *beyond = (beyond.0 + x, beyond.1 + y);
}

/// A new state of the mouse buttons (bit 0 left, 1 right), for the
/// program's handler to hear in turn.
pub fn mouse_button_state(cpu: &mut Cpu, buttons: u32) {
    const MOST_WAITING: usize = 16;
    if cpu.host.dos.mouse_handler.is_some() && cpu.host.dos.mouse_buttons.len() < MOST_WAITING {
        cpu.host.dos.mouse_buttons.push_back(buttons);
    }
}

/// Tells the program's mouse handler what changed since it was last told,
/// as the driver did from its interrupt: AX the events (bit 0 moved, 1 and
/// 2 left press and release, 3 and 4 right press and release), BX the
/// buttons, CX and DX the place, SI and DI the running mickey counts. The
/// handler returns with a far `ret`. Called at the program's safe points.
/// Extreme Assault's moves its own pointer by the change in SI and DI, and
/// ignores CX and DX, so the window holds the mouse meanwhile, and the two
/// pointers move together.
pub fn mouse_events(cpu: &mut Cpu) {
    let Some((handler, wanted)) = cpu.host.dos.mouse_handler else { return };
    super::video::capture_mouse(cpu);
    let (x, y) = cpu.host.pointer;
    let (told_x, told_y, told_buttons) = cpu.host.dos.mouse_told;
    let waited = cpu.host.dos.mouse_button_told.is_none_or(|told| told.elapsed() >= BUTTON_HELD);
    let queued = if waited { cpu.host.dos.mouse_buttons.pop_front() } else { None };
    if queued.is_some() {
        cpu.host.dos.mouse_button_told = Some(std::time::Instant::now());
    }
    // While queued states wait their turn, the last one told still holds.
    let buttons = queued.unwrap_or_else(|| if cpu.host.dos.mouse_buttons.is_empty() { super::video::mouse_buttons(cpu) } else { told_buttons });
    let (mickeys_x, mickeys_y) = cpu.host.dos.mickeys;
    let started = std::mem::replace(&mut cpu.host.dos.mouse_started, true);
    // The first call only sets the handler's starting counts.
    let (beyond_x, beyond_y) = if started { std::mem::take(&mut cpu.host.dos.mickeys_beyond) } else { (0, 0) };
    let mickeys = (
        mickeys_x.wrapping_add(x.wrapping_sub(told_x)).wrapping_add(beyond_x as u32),
        mickeys_y.wrapping_add(y.wrapping_sub(told_y)).wrapping_add(beyond_y as u32),
    );
    let mut events = 0;
    if !started || mickeys != (mickeys_x, mickeys_y) {
        events |= 1;
    }
    cpu.host.dos.mickeys = if started { mickeys } else { (mickeys_x, mickeys_y) };
    let mickeys = cpu.host.dos.mickeys;
    for (button, press) in [(1, 2), (2, 8)] {
        let (now, before) = (buttons & button != 0, told_buttons & button != 0);
        if now && !before {
            events |= press;
        } else if !now && before {
            events |= press << 1;
        }
    }
    cpu.host.dos.mouse_told = (x, y, buttons);
    if events & wanted == 0 {
        return;
    }
    let saved = super::timers::Registers::of(cpu);
    cpu.eax = events & wanted;
    cpu.ebx = buttons;
    cpu.ecx = x;
    cpu.edx = y;
    cpu.esi = mickeys.0 & 0xffff;
    cpu.edi = mickeys.1 & 0xffff;
    cpu.df = false;
    cpu.push(FLAT as u32);
    cpu.push(0);
    if cpu.trace {
        println!("  mouse handler {handler:#x}: events {:#x} at {x},{y} buttons {buttons}", events & wanted);
    }
    cpu.call(handler);
    saved.restore(cpu);
}

/// A path the program gave, with the current folder in front when it's
/// relative: neither on a drive nor from the root.
fn full_path(cpu: &Cpu, name: &str) -> String {
    let folder = &cpu.host.dos.folder;
    let absolute = name.starts_with(['\\', '/']) || name.as_bytes().get(1) == Some(&b':');
    if absolute || folder.is_empty() {
        name.to_string()
    } else {
        format!("\\{folder}\\{name}")
    }
}

/// The path at DS:EDX, as `full_path` makes it.
fn path_argument(cpu: &Cpu) -> String {
    let name = cpu.string(at(cpu, DS, cpu.edx));
    full_path(cpu, &name)
}

/// The folder `int 21h` AH=3Bh changes to, from the current one: its parts
/// from the root, with `.` and `..` taken out. A drive name is dropped, as
/// every drive's folders are the game folder's.
fn changed_folder(current: &str, name: &str) -> String {
    let name = name.replace('/', "\\");
    let name = if name.as_bytes().get(1) == Some(&b':') { &name[2..] } else { &name[..] };
    let mut parts: Vec<&str> = if name.starts_with('\\') { Vec::new() } else { current.split('\\').filter(|part| !part.is_empty()).collect() };
    for part in name.split('\\').filter(|part| !part.is_empty() && *part != ".") {
        if part == ".." {
            parts.pop();
        } else {
            parts.push(part);
        }
    }
    parts.join("\\")
}

/// What an interrupt vector held before the program set it: DOS's or the
/// BIOS's handler, which has nothing left to do here, as the runtime does
/// their work itself. A handler that chains to it, by a far call after
/// `pushfd` or a far jump, comes back as if from `iretd`.
fn previous_handler(cpu: &mut Cpu) {
    cpu.returned_to = cpu.pop();
    cpu.pop();
    let flags = cpu.pop();
    cpu.set_flags(flags);
}

/// The vector's handler: the program's, or the previous one.
fn vector(cpu: &Cpu, number: u8) -> (u16, u32) {
    cpu.host.dos.vectors.get(&number).copied().unwrap_or((FLAT, cpu.host.dos.previous))
}

/// An address the program passed in a segment and an offset register.
fn at(cpu: &Cpu, segment: usize, offset: u32) -> u32 {
    cpu.bases[segment].wrapping_add(offset)
}

/// Sets or clears the carry flag, DOS's sign of an error, with the error code in AX.
fn result(cpu: &mut Cpu, outcome: Result<u32, u32>) {
    match outcome {
        Ok(value) => {
            set_ax(cpu, value);
            cpu.cf = false;
        }
        Err(code) => {
            set_ax(cpu, code);
            cpu.cf = true;
        }
    }
}

const FILE_NOT_FOUND: u32 = 2;
const PATH_NOT_FOUND: u32 = 3;
const BAD_HANDLE: u32 = 6;
const NO_MORE_FILES: u32 = 0x12;
const FIRST_HANDLE: u16 = 5;

fn new_handle(cpu: &mut Cpu, host: u32) -> u32 {
    let handle = (FIRST_HANDLE..).find(|h| !cpu.host.dos.handles.contains_key(h)).unwrap();
    cpu.host.dos.handles.insert(handle, host);
    handle as u32
}

/// `int 21h` AH=3Ch, 3Dh: creates or opens the file named at DS:EDX.
fn open(cpu: &mut Cpu, create: bool) -> Result<u32, u32> {
    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const OPEN_EXISTING: u32 = 3;
    const CREATE_ALWAYS: u32 = 2;
    const INVALID: u32 = 0xffff_ffff;
    let name = path_argument(cpu);
    // Miles' settings for its sound card drivers, which `SETUP.EXE` wrote.
    // The drivers are real-mode code the runtime can't run, and a mod plays
    // the sound instead (Extreme Assault's `mods/miles.rs`), so whatever the
    // folder holds, these are a stand-in for a card that's there. Extreme
    // Assault speaks its briefings only when `DRIVERS\DIG.INI` is there, and
    // a disc that was never set up has none.
    let file = name.rsplit(['\\', '/']).next().unwrap_or_default();
    if !create && (file.eq_ignore_ascii_case("MDI.INI") || file.eq_ignore_ascii_case("DIG.INI")) {
        let stand_in = b"DEVICE_NAME Recompiled runtime\r\nDRIVER_NAME RUNTIME\r\nIO_ADDR 220\r\nIRQ 7\r\nDMA_8_BIT 1\r\nDMA_16_BIT 5\r\n".to_vec();
        let host = super::open_memory(cpu, std::path::PathBuf::from(&name), stand_in);
        return Ok(new_handle(cpu, host));
    }
    let writing = create || cpu.eax & 3 != 0;
    // Extreme Assault's `LORESOLUTION_METHODE` picks a software video mode
    // for its menus. A copy set up on another machine may name one the
    // runtime doesn't offer, such as 512 from a DOSBox copy: the game's test
    // for it fails, it falls back to a high-resolution value, and its menus
    // then never take input. The game is read 32 instead, the disc's own
    // value, and the file on disk is left alone.
    //
    // Its `SPEAK` picks the briefings' speech: 0 is `DATA.SFX\SPEAK_D`, 1 is
    // `SPEAK_E`. The English disc's own file says 0, but the disc only has
    // `SPEAK_E` (`SETUP.EXE` would have set 1). Without the speech, each of
    // the briefing's first texts ends before it shows, and only the later
    // ones near the top of the screen appear. The game is read the language
    // whose folder is there.
    if !writing && file.eq_ignore_ascii_case("INIT.CFG") {
        if let Ok(text) = std::fs::read(super::game_path(cpu, &name)) {
            let speech_there = |letter: char| super::game_path(cpu, &format!("DATA.SFX\\SPEAK_{letter}")).is_dir();
            let speech = [speech_there('D'), speech_there('E')];
            let fixed = usable_video_mode(&text);
            let fixed = speech_with_folder(fixed.as_deref().unwrap_or(&text), speech).or(fixed);
            if let Some(fixed) = fixed {
                let host = super::open_memory(cpu, std::path::PathBuf::from(&name), fixed);
                return Ok(new_handle(cpu, host));
            }
        }
    }
    let access = GENERIC_READ | if writing { GENERIC_WRITE } else { 0 };
    let host = super::kernel32::create_file(cpu, name.clone(), access, if create { CREATE_ALWAYS } else { OPEN_EXISTING });
    if host == INVALID {
        println!("  can't open {name}");
        return Err(FILE_NOT_FOUND);
    }
    Ok(new_handle(cpu, host))
}

/// The low-resolution video modes the runtime offers Extreme Assault, as its
/// `LORESOLUTION_METHODE` bits: its own start-up test finds these.
const USABLE_LOW_MODES: u32 = 0x1 | 0x2 | 0x4 | 0x8 | 0x20 | 0x80 | 0x100;
const LOW_MODE_SETTING: &[u8] = b"LORESOLUTION_METHODE=";

/// `INIT.CFG` with its `LORESOLUTION_METHODE` changed to 32 when it names a
/// mode the runtime doesn't offer, keeping everything else byte for byte.
/// `None` when the file is fine as it is.
fn usable_video_mode(text: &[u8]) -> Option<Vec<u8>> {
    let start = text.windows(LOW_MODE_SETTING.len()).position(|window| window == LOW_MODE_SETTING)? + LOW_MODE_SETTING.len();
    let end = start + text[start..].iter().take_while(|byte| byte.is_ascii_digit()).count();
    let mode: u32 = std::str::from_utf8(&text[start..end]).ok()?.parse().ok()?;
    if mode.is_power_of_two() && mode & USABLE_LOW_MODES != 0 {
        return None;
    }
    println!("INIT.CFG names video mode {mode}, which isn't offered here: the game is given 32");
    Some([&text[..start], b"32", &text[end..]].concat())
}

const SPEAK_SETTING: &[u8] = b"\nSPEAK=";

/// `INIT.CFG` with its `SPEAK` changed to the language whose speech folder
/// is there (`there` is `[SPEAK_D, SPEAK_E]`), when the one it names isn't.
/// `None` when the file is fine as it is, or neither folder is there.
fn speech_with_folder(text: &[u8], there: [bool; 2]) -> Option<Vec<u8>> {
    let start = text.windows(SPEAK_SETTING.len()).position(|window| window == SPEAK_SETTING)? + SPEAK_SETTING.len();
    let end = start + text[start..].iter().take_while(|byte| byte.is_ascii_digit()).count();
    let speak: usize = std::str::from_utf8(&text[start..end]).ok()?.parse().ok()?;
    if there.get(speak) == Some(&true) {
        return None;
    }
    let other = there.iter().position(|&is| is)?;
    println!("INIT.CFG names speech {speak}, whose folder isn't there: the game is given {other}");
    Some([&text[..start], other.to_string().as_bytes(), &text[end..]].concat())
}

#[cfg(test)]
mod tests {
    use super::{speech_with_folder, usable_video_mode};

    #[test]
    fn speech_follows_the_folder_there() {
        assert_eq!(speech_with_folder(b"A=1\r\nSPEAK=0\r\nB=2\r\n", [false, true]).unwrap(), b"A=1\r\nSPEAK=1\r\nB=2\r\n");
        assert_eq!(speech_with_folder(b"A=1\r\nSPEAK=1\r\n", [true, false]).unwrap(), b"A=1\r\nSPEAK=0\r\n");
        assert_eq!(speech_with_folder(b"A=1\r\nSPEAK=0\r\n", [true, true]), None);
        assert_eq!(speech_with_folder(b"A=1\r\nSPEAK=0\r\n", [false, false]), None);
        assert_eq!(speech_with_folder(b"NOTHING=1\r\n", [false, true]), None);
    }

    #[test]
    fn unusable_video_mode_becomes_32() {
        assert_eq!(usable_video_mode(b"A=1\r\nLORESOLUTION_METHODE=512\r\nB=2\r\n").unwrap(), b"A=1\r\nLORESOLUTION_METHODE=32\r\nB=2\r\n");
        assert_eq!(usable_video_mode(b"LORESOLUTION_METHODE=32\r\n"), None);
        assert_eq!(usable_video_mode(b"LORESOLUTION_METHODE=256\n"), None);
        assert_eq!(usable_video_mode(b"NOTHING=1\r\n"), None);
    }
}

fn host_handle(cpu: &Cpu) -> Option<u32> {
    cpu.host.dos.handles.get(&(cpu.ebx as u16)).copied()
}

/// `int 21h` AH=42h: moves a file's position, CX:DX from where AL says.
fn seek(cpu: &mut Cpu) -> Result<u32, u32> {
    use std::io::{Seek, SeekFrom};
    let handle = host_handle(cpu).ok_or(BAD_HANDLE)?;
    let distance = ((cpu.ecx & 0xffff) << 16 | cpu.edx & 0xffff) as i32 as i64;
    let target = match cpu.eax & 0xff {
        0 => SeekFrom::Start(distance as u64),
        1 => SeekFrom::Current(distance),
        _ => SeekFrom::End(distance),
    };
    let open = cpu.host.files.get_mut(&handle).ok_or(BAD_HANDLE)?;
    let position = open.file.seek(target).map_err(|_| BAD_HANDLE)? as u32;
    cpu.edx = (cpu.edx & !0xffff) | position >> 16;
    Ok(position & 0xffff)
}

/// `int 21h` AH=4Eh, 4Fh: files matching DS:EDX, written to the transfer area.
fn find(cpu: &mut Cpu, first: bool) -> Result<u32, u32> {
    if first {
        let pattern = path_argument(cpu).replace('\\', "/");
        let (folder, mask) = pattern.rsplit_once('/').unwrap_or(("", &pattern));
        let directories = cpu.ecx & 0x10 != 0;
        let path = super::kernel32::host_path(cpu, if folder.is_empty() { "." } else { folder });
        let mut found: Vec<(String, bool, u64)> = std::fs::read_dir(path)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().to_uppercase();
                let metadata = entry.metadata().ok()?;
                (matches(mask, &name) && (directories || !metadata.is_dir())).then_some((name, metadata.is_dir(), metadata.len()))
            })
            .collect();
        found.sort();
        found.reverse();
        cpu.host.dos.found = found;
    }
    let (name, directory, size) = cpu.host.dos.found.pop().ok_or(if first { FILE_NOT_FOUND } else { NO_MORE_FILES })?;
    let area = cpu.host.dos.transfer_area;
    cpu.fill(area, 43, 0);
    cpu.w8(area + 0x15, if directory { 0x10 } else { 0x20 });
    cpu.w32(area + 0x1a, size as u32);
    cpu.write_string(area + 0x1e, &name);
    Ok(0)
}

/// DOS's wildcards: `*` up to the dot or the end, `?` any one character.
fn matches(mask: &str, name: &str) -> bool {
    let split = |text: &str| -> (String, String) {
        let (base, extension) = text.split_once('.').unwrap_or((text, ""));
        (base.to_uppercase(), extension.to_uppercase())
    };
    let part = |mask: &str, text: &str| -> bool {
        let (mask, text): (Vec<char>, Vec<char>) = (mask.chars().collect(), text.chars().collect());
        let mut index = 0;
        for (position, &m) in mask.iter().enumerate() {
            match m {
                '*' => return true,
                '?' => index += 1,
                c if text.get(index) == Some(&c) => index += 1,
                _ => {
                    let _ = position;
                    return false;
                }
            }
        }
        index >= text.len()
    };
    let ((mask_base, mask_extension), (base, extension)) = (split(mask), split(name));
    part(&mask_base, &base) && part(if mask.contains('.') { &mask_extension } else { "*" }, &extension)
}

/// Where DOS memory blocks start, in the first megabyte, and where it ends:
/// below the VGA memory.
const DOS_MEMORY: u32 = 0x10000;
const DOS_MEMORY_END: u32 = 0xa0000;
const FIRST_SELECTOR: u16 = 0x80;
const SELECTOR_STEP: u16 = 8;

fn new_selector(cpu: &mut Cpu, base: u32) -> u16 {
    let selector = (0..).map(|n| FIRST_SELECTOR + n * SELECTOR_STEP).find(|s| !cpu.host.dos.selectors.contains_key(s)).unwrap();
    cpu.host.dos.selectors.insert(selector, base);
    selector
}

/// `int 31h`: DPMI, as DOS/4GW gives it.
fn dpmi_call(cpu: &mut Cpu, ax: u32) -> bool {
    let (bx, cx, dx) = (cpu.ebx & 0xffff, cpu.ecx & 0xffff, cpu.edx & 0xffff);
    let split = |value: u32| (value >> 16, value & 0xffff);
    match ax {
        // Descriptors: allocate CX of them, free one, the step between them,
        // set a base, a limit or rights, alias one.
        0x0000 => {
            let first = new_selector(cpu, 0);
            for _ in 1..cx.max(1) {
                new_selector(cpu, 0);
            }
            set_ax(cpu, first as u32);
        }
        0x0001 => {
            cpu.host.dos.selectors.remove(&(bx as u16));
        }
        0x0003 => set_ax(cpu, SELECTOR_STEP as u32),
        0x0007 => {
            cpu.host.dos.selectors.insert(bx as u16, cx << 16 | dx);
        }
        0x0008 | 0x0009 => {}
        0x000a => {
            let base = selector_base(cpu, bx as u16);
            let alias = new_selector(cpu, base);
            set_ax(cpu, alias as u32);
        }
        // DOS memory: BX paragraphs, at a real-mode segment in AX with a
        // selector in DX. It's never given back: games ask for little.
        0x0100 => {
            let size = bx << 4;
            let start = cpu.host.dos.dos_next.max(DOS_MEMORY);
            if start + size > DOS_MEMORY_END {
                set_ax(cpu, 8);
                cpu.ebx = (cpu.ebx & !0xffff) | (DOS_MEMORY_END - start) >> 4;
                cpu.cf = true;
                return true;
            }
            cpu.host.dos.dos_next = (start + size + 15) & !15;
            cpu.fill(start, size, 0);
            let selector = new_selector(cpu, start);
            set_ax(cpu, start >> 4);
            cpu.edx = (cpu.edx & !0xffff) | selector as u32;
        }
        0x0101 | 0x0102 => {}
        // Real-mode interrupt vectors, and the program's own handlers for
        // processor exceptions and interrupts.
        0x0200 => {
            cpu.ecx &= !0xffff;
            cpu.edx &= !0xffff;
        }
        0x0201 | 0x0202 | 0x0203 => {}
        0x0204 => {
            let (selector, offset) = vector(cpu, bx as u8);
            cpu.ecx = (cpu.ecx & !0xffff) | selector as u32;
            cpu.edx = offset;
        }
        0x0205 => {
            cpu.host.dos.vectors.insert(bx as u8, (cx as u16, cpu.edx));
        }
        // A real-mode interrupt with the registers at ES:EDI.
        0x0300 => real_mode_interrupt(cpu, bx as u8, at(cpu, ES, cpu.edi)),
        // DPMI 0.9 on a 486, with the interrupt controllers where DOS has them.
        0x0400 => {
            set_ax(cpu, 0x005a);
            cpu.ebx = (cpu.ebx & !0xffff) | 0x0005;
            cpu.ecx = (cpu.ecx & !0xff) | 4;
            cpu.edx = (cpu.edx & !0xffff) | 0x0870;
        }
        // Free memory: plenty.
        0x0500 => {
            let info = at(cpu, ES, cpu.edi);
            const PLENTY: u32 = 64 << 20;
            const PAGE: u32 = 4096;
            cpu.fill(info, 0x30, 0xff);
            cpu.w32(info, PLENTY);
            cpu.w32(info + 4, PLENTY / PAGE);
            cpu.w32(info + 8, PLENTY / PAGE);
            cpu.w32(info + 0x14, PLENTY / PAGE);
        }
        // Memory blocks, BX:CX bytes, at the linear address in BX:CX, with
        // the same as the handle in SI:DI.
        0x0501 => {
            let size = bx << 16 | cx;
            let block = cpu.allocate(size);
            cpu.fill(block, size, 0);
            let (high, low) = split(block);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
            cpu.esi = (cpu.esi & !0xffff) | high;
            cpu.edi = (cpu.edi & !0xffff) | low;
        }
        0x0502 => {}
        0x0503 => {
            let (size, old) = (bx << 16 | cx, (cpu.esi & 0xffff) << 16 | cpu.edi & 0xffff);
            let block = cpu.allocate(size);
            let keep = cpu.r32(old.wrapping_sub(12)).min(size);
            let data = cpu.bytes(old, keep).to_vec();
            cpu.bytes_mut(block, keep).copy_from_slice(&data);
            let (high, low) = split(block);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
            cpu.esi = (cpu.esi & !0xffff) | high;
            cpu.edi = (cpu.edi & !0xffff) | low;
        }
        // Physical memory: the VESA frame buffer, BX:CX, to a linear address.
        0x0800 => {
            let linear = super::vga::map_physical(cpu, bx << 16 | cx);
            let (high, low) = split(linear);
            cpu.ebx = (cpu.ebx & !0xffff) | high;
            cpu.ecx = (cpu.ecx & !0xffff) | low;
        }
        0x0801 => {}
        // Locking memory, and the virtual interrupt flag: nothing to do.
        0x0600 | 0x0601 | 0x0602 | 0x0603 => {}
        // The page size, in BX:CX.
        0x0604 => {
            cpu.ebx &= !0xffff;
            cpu.ecx = (cpu.ecx & !0xffff) | 4096;
        }
        0x0900 | 0x0901 => set_al(cpu, 1),
        0x0902 => set_al(cpu, 1),
        _ => return false,
    }
    cpu.cf = false;
    true
}

/// A real-mode interrupt, with registers as DPMI's 0x32-byte structure at
/// `registers`: EDI, ESI, EBP, -, EBX, EDX, ECX, EAX, flags, ES, DS, ...
/// Buffers are at a real-mode segment and offset.
fn real_mode_interrupt(cpu: &mut Cpu, number: u8, registers: u32) {
    const EDI: u32 = 0;
    const EBX: u32 = 0x10;
    const EDX: u32 = 0x14;
    const ECX: u32 = 0x18;
    const EAX: u32 = 0x1c;
    const FLAGS: u32 = 0x20;
    const SEGMENT_ES: u32 = 0x22;
    const CARRY: u16 = 1;
    let segment = cpu.r16(registers + SEGMENT_ES) as u32;
    let edi = cpu.r32(registers + EDI);
    let mut r = super::vga::Registers {
        eax: cpu.r32(registers + EAX),
        ebx: cpu.r32(registers + EBX),
        ecx: cpu.r32(registers + ECX),
        edx: cpu.r32(registers + EDX),
        edi,
        buffer: segment * 16 + (edi & 0xffff),
        segment,
    };
    const ESI: u32 = 0x04;
    const SEGMENT_DS: u32 = 0x24;
    let handled = match (number, r.eax & 0xffff) {
        (0x10, _) => super::vga::bios(cpu, &mut r),
        (0x2f, ax) if ax >> 8 == 0x15 => {
            let buffer = r.segment * 16 + (r.ebx & 0xffff);
            super::mscdex::call(cpu, &mut r, buffer)
        }
        // DOS's table of lead bytes for double-byte characters: empty, two
        // zero bytes in the PSP.
        (0x21, 0x6300) => {
            const EMPTY: u32 = 0xfe;
            cpu.w16(PSP + EMPTY, 0);
            cpu.w16(registers + SEGMENT_DS, (PSP >> 4) as u16);
            cpu.w32(registers + ESI, EMPTY);
            r.eax &= !0xff;
            true
        }
        _ => false,
    };
    if !handled {
        panic!("the runtime doesn't have real-mode int {number:02x}h with ax={:04x} bx={:04x} cx={:04x} dx={:04x} yet", r.eax & 0xffff, r.ebx & 0xffff, r.ecx & 0xffff, r.edx & 0xffff);
    }
    cpu.w32(registers + EAX, r.eax);
    cpu.w32(registers + EBX, r.ebx);
    cpu.w32(registers + ECX, r.ecx);
    cpu.w32(registers + EDX, r.edx);
    cpu.w32(registers + EDI, r.edi);
    let flags = cpu.r16(registers + FLAGS) & !CARRY;
    cpu.w16(registers + FLAGS, flags);
}

/// `int 10h` from protected mode: buffers are at ES:EDI.
fn video_bios(cpu: &mut Cpu) -> bool {
    let mut r = super::vga::Registers {
        eax: cpu.eax,
        ebx: cpu.ebx,
        ecx: cpu.ecx,
        edx: cpu.edx,
        edi: cpu.edi,
        buffer: at(cpu, ES, cpu.edi),
        segment: 0,
    };
    // Function 10h/12h takes its colours at ES:DX.
    if cpu.eax & 0xffff == 0x1012 {
        r.buffer = at(cpu, ES, cpu.edx);
    }
    if !super::vga::bios(cpu, &mut r) {
        return false;
    }
    (cpu.eax, cpu.ebx, cpu.ecx, cpu.edx, cpu.edi) = (r.eax, r.ebx, r.ecx, r.edx, r.edi);
    true
}

/// `int 33h`: the mouse driver. The pointer is the runtime's, on a 640x480
/// screen, and movement is reported as the change in its position.
fn mouse_call(cpu: &mut Cpu, ax: u32) -> bool {
    let (x, y) = cpu.host.pointer;
    if cpu.trace_filter.as_deref() == Some("mouse") && ax != 0x03 && ax != 0x0b {
        println!("  int 33h {ax:#x} cx {:#x} edx {:#x}", cpu.ecx & 0xffff, cpu.edx);
    }
    match ax {
        // Reset: a driver is there, with two buttons.
        0x00 | 0x21 => {
            set_ax(cpu, 0xffff);
            cpu.ebx = (cpu.ebx & !0xffff) | 2;
            cpu.host.dos.mouse_seen = (x, y);
            cpu.host.dos.mouse_handler = None;
        }
        // Show and hide the pointer, which only text modes draw.
        0x01 | 0x02 => {}
        0x03 => {
            cpu.ebx = (cpu.ebx & !0xffff) | super::video::mouse_buttons(cpu);
            cpu.ecx = (cpu.ecx & !0xffff) | x;
            cpu.edx = (cpu.edx & !0xffff) | y;
        }
        0x04 => cpu.host.pointer = ((cpu.ecx & 0xffff).min(639), (cpu.edx & 0xffff).min(479)),
        // Presses and releases of one button since the last call: none kept.
        0x05 | 0x06 => {
            set_ax(cpu, super::video::mouse_buttons(cpu));
            cpu.ebx &= !0xffff;
            cpu.ecx = (cpu.ecx & !0xffff) | x;
            cpu.edx = (cpu.edx & !0xffff) | y;
        }
        // The program's handler, at ES:EDX, for the events in CX: see
        // `mouse_events`. A mask of 0 takes it away.
        0x0c => {
            let mask = cpu.ecx & 0xffff;
            let handler = at(cpu, ES, cpu.edx);
            cpu.host.dos.mouse_handler = (mask != 0 && handler != 0).then_some((handler, mask));
            cpu.host.dos.mouse_told = (x, y, super::video::mouse_buttons(cpu));
            cpu.host.dos.mouse_started = false;
        }
        // Limits, the pointer's shape, speed: taken and ignored.
        0x07 | 0x08 | 0x09 | 0x0a | 0x0f | 0x10 | 0x13 | 0x14 | 0x1a | 0x1c | 0x1d => {}
        // How far the mouse moved since the last call, in mickeys.
        0x0b => {
            let (seen_x, seen_y) = cpu.host.dos.mouse_seen;
            cpu.host.dos.mouse_seen = (x, y);
            cpu.ecx = (cpu.ecx & !0xffff) | (x.wrapping_sub(seen_x) & 0xffff);
            cpu.edx = (cpu.edx & !0xffff) | (y.wrapping_sub(seen_y) & 0xffff);
        }
        _ => return false,
    }
    true
}

/// `int 2Fh` AH=15h from protected mode: the CD driver, with buffers at ES:EBX.
fn cd_call(cpu: &mut Cpu) -> bool {
    let mut r = super::vga::Registers { eax: cpu.eax, ebx: cpu.ebx, ecx: cpu.ecx, edx: cpu.edx, edi: cpu.edi, buffer: 0, segment: 0 };
    let buffer = at(cpu, ES, cpu.ebx);
    if !super::mscdex::call(cpu, &mut r, buffer) {
        return false;
    }
    (cpu.eax, cpu.ebx, cpu.ecx, cpu.edx) = (r.eax, r.ebx, r.ecx, r.edx);
    true
}

/// `int 21h`: DOS.
fn dos_call(cpu: &mut Cpu, ah: u32) -> bool {
    use std::io::Write;
    match ah {
        // Starting another program. The game runs SciTech's UVCONFIG.EXE
        // when `univbe.drv` is missing, to probe the graphics card. The
        // runtime offers VBE 2.0 itself, so there's nothing to run: DOS's
        // answer for a missing program, error 2.
        0x4b => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            println!("  not running {name}");
            cpu.eax = (cpu.eax & !0xffff) | 2;
            cpu.cf = true;
        }
        // Whether a key waits at DOS's console: none. Keys reach the game as
        // scan codes through its keyboard handler (`host::pc`), not here.
        0x0b => set_al(cpu, 0),
        // The current drive: C:.
        0x19 => set_al(cpu, 2),
        // Selecting a drive: there are 26.
        0x0e => set_al(cpu, 26),
        // The disk transfer area, for finding files.
        0x1a => cpu.host.dos.transfer_area = at(cpu, DS, cpu.edx),
        0x2f => {
            let area = cpu.host.dos.transfer_area;
            cpu.ebx = area.wrapping_sub(cpu.bases[ES]);
        }
        // Interrupt vectors the program sets and reads: a selector and offset.
        0x25 => {
            let (number, selector, offset) = (cpu.eax as u8, cpu.segments[DS], cpu.edx);
            cpu.host.dos.vectors.insert(number, (selector, offset));
        }
        0x35 => {
            let (selector, offset) = vector(cpu, cpu.eax as u8);
            cpu.set_segment(ES, selector);
            cpu.ebx = offset;
        }
        // Date and time.
        0x2a | 0x2c => {
            let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
            let (days, rest) = (seconds / 86400, seconds % 86400);
            if ah == 0x2c {
                cpu.ecx = (cpu.ecx & !0xffff) | ((rest / 3600) as u32) << 8 | ((rest / 60) % 60) as u32;
                cpu.edx = (cpu.edx & !0xffff) | ((rest % 60) as u32) << 8;
            } else {
                // Days since 1970 to a date, by the civil calendar.
                let z = days as i64 + 719_468;
                let era = z.div_euclid(146_097);
                let day = z - era * 146_097;
                let year_of_era = (day - day / 1460 + day / 36524 - day / 146_096) / 365;
                let day_of_year = day - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
                let month_part = (5 * day_of_year + 2) / 153;
                let date = day_of_year - (153 * month_part + 2) / 5 + 1;
                let month = if month_part < 10 { month_part + 3 } else { month_part - 9 };
                let year = year_of_era + era * 400 + i64::from(month <= 2);
                cpu.ecx = (cpu.ecx & !0xffff) | year as u32;
                cpu.edx = (cpu.edx & !0xffff) | (month as u32) << 8 | date as u32;
                set_al(cpu, ((days + 4) % 7) as u32);
            }
        }
        0x3c | 0x3d => {
            let outcome = open(cpu, ah == 0x3c);
            result(cpu, outcome);
        }
        0x3e => {
            let outcome = match cpu.host.dos.handles.remove(&(cpu.ebx as u16)) {
                Some(host) => {
                    cpu.host.files.remove(&host);
                    Ok(cpu.eax)
                }
                None if (cpu.ebx as u16) < FIRST_HANDLE => Ok(cpu.eax),
                None => Err(BAD_HANDLE),
            };
            result(cpu, outcome);
        }
        0x3f => {
            let outcome = match host_handle(cpu) {
                Some(host) => {
                    let target = at(cpu, DS, cpu.edx);
                    let count = super::kernel32::read_file(cpu, host, target, cpu.ecx);
                    cpu.eax = count;
                    cpu.cf = false;
                    return true;
                }
                // Standard input: nothing typed.
                None if (cpu.ebx as u16) < FIRST_HANDLE => Ok(0),
                None => Err(BAD_HANDLE),
            };
            result(cpu, outcome);
        }
        0x40 => {
            let data = cpu.bytes(at(cpu, DS, cpu.edx), cpu.ecx).to_vec();
            let outcome = match host_handle(cpu) {
                Some(host) => match cpu.host.files.get_mut(&host) {
                    Some(open) => open.file.write_all(&data).map(|_| data.len() as u32).map_err(|_| BAD_HANDLE),
                    None => Err(BAD_HANDLE),
                },
                // Standard output and error go to the log.
                None if (cpu.ebx as u16) < FIRST_HANDLE => {
                    print!("{}", String::from_utf8_lossy(&data));
                    Ok(data.len() as u32)
                }
                None => Err(BAD_HANDLE),
            };
            if let Ok(count) = outcome {
                cpu.eax = count;
                cpu.cf = false;
            } else {
                result(cpu, outcome);
            }
        }
        0x41 => {
            let name = path_argument(cpu);
            let path = super::kernel32::host_path(cpu, &name);
            let outcome = std::fs::remove_file(path).map(|_| 0).map_err(|_| FILE_NOT_FOUND);
            result(cpu, outcome);
        }
        0x42 => {
            let outcome = seek(cpu);
            result(cpu, outcome);
        }
        // A file's attributes: whether it's there.
        0x43 => {
            let name = path_argument(cpu);
            let path = super::kernel32::host_path(cpu, &name);
            let outcome = match std::fs::metadata(path) {
                Ok(metadata) => {
                    cpu.ecx = (cpu.ecx & !0xffff) | if metadata.is_dir() { 0x10 } else { 0x20 };
                    Ok(cpu.eax)
                }
                Err(_) => Err(FILE_NOT_FOUND),
            };
            result(cpu, outcome);
        }
        // Device information: the standard handles are the console, the
        // others files on drive C:.
        0x44 if cpu.eax & 0xff == 0 => {
            let handle = cpu.ebx as u16;
            let info = if handle < FIRST_HANDLE { 0x80d3 } else if cpu.host.dos.handles.contains_key(&handle) { 0x0002 } else { 0 };
            cpu.edx = (cpu.edx & !0xffff) | info;
            result(cpu, if info == 0 { Err(BAD_HANDLE) } else { Ok(cpu.eax) });
        }
        // The current folder, with no drive and no `\` in front.
        0x47 => {
            let target = at(cpu, DS, cpu.esi);
            let folder = cpu.host.dos.folder.clone();
            cpu.write_string(target, &folder);
            result(cpu, Ok(cpu.eax));
        }
        // Changing the current folder.
        0x3b => {
            let name = cpu.string(at(cpu, DS, cpu.edx));
            let folder = changed_folder(&cpu.host.dos.folder, &name);
            let outcome = if super::kernel32::host_path(cpu, &format!("\\{folder}")).is_dir() {
                if cpu.trace {
                    println!("  current folder \\{folder}");
                }
                cpu.host.dos.folder = folder;
                Ok(cpu.eax)
            } else {
                Err(PATH_NOT_FOUND)
            };
            result(cpu, outcome);
        }
        0x4e | 0x4f => {
            let outcome = find(cpu, ah == 0x4e);
            result(cpu, outcome);
        }
        0x4c => {
            println!("the game exited with code {}", cpu.eax & 0xff);
            std::process::exit((cpu.eax & 0xff) as i32);
        }
        _ => return false,
    }
    true
}

/// A selector's base, for `Cpu::set_segment`. Unknown selectors are taken as
/// flat, and logged.
pub fn selector_base(cpu: &mut Cpu, selector: u16) -> u32 {
    if selector == FLAT || selector == 0 {
        return 0;
    }
    match cpu.host.dos.selectors.get(&selector) {
        Some(&base) => base,
        None => {
            println!("unknown selector {selector:#x}, taken as flat");
            cpu.host.dos.selectors.insert(selector, 0);
            0
        }
    }
}

/// Sets up what DOS and DOS/4GW give a program before its first instruction:
/// the PSP with the command line, the environment, the selectors and the
/// registers. `program` is the path DOS would name in the environment.
pub fn start(cpu: &mut Cpu, stack: u32, program: &str, arguments: &str) {
    cpu.host.dos.selectors.insert(PSP_SELECTOR, PSP);
    cpu.host.dos.selectors.insert(ENVIRONMENT_SELECTOR, ENVIRONMENT);

    // The PSP: `int 20h` at its start, the environment's selector at 0x2C,
    // and the command line's length and text at 0x80.
    cpu.fill(PSP, 0x100, 0);
    cpu.w16(PSP, 0x20cd);
    cpu.w16(PSP + 0x2c, ENVIRONMENT_SELECTOR);
    let tail: String = format!(" {arguments}").chars().take(126).collect();
    let tail = if arguments.is_empty() { String::new() } else { tail };
    cpu.w8(PSP + 0x80, tail.len() as u8);
    cpu.write_string(PSP + 0x81, &tail);
    cpu.w8(PSP + 0x81 + tail.len() as u32, 0x0d);

    // The environment: strings, an empty one, a count of 1, then the
    // program's own path.
    let mut at = ENVIRONMENT;
    for variable in ["COMSPEC=C:\\COMMAND.COM", "PATH=C:\\", "DOS4G=QUIET"] {
        cpu.write_string(at, variable);
        at += variable.len() as u32 + 1;
    }
    cpu.w8(at, 0);
    cpu.w16(at + 1, 1);
    cpu.write_string(at + 3, program);

    for register in [CS, SS, DS, FS] {
        cpu.set_segment(register, FLAT);
    }
    cpu.set_segment(ES, PSP_SELECTOR);
    cpu.set_segment(GS, 0);
    cpu.esp = stack;
    // DOS's default transfer area is in the PSP.
    cpu.host.dos.transfer_area = PSP + 0x80;
    cpu.imports.push(crate::cpu::Host { name: "dos!previous_handler".to_string(), function: Some(previous_handler) });
    cpu.host.dos.previous = crate::cpu::IMPORT_BASE + cpu.imports.len() as u32 - 1;
    // The PC's hardware is serviced at the program's safe points.
    cpu.host.timers.dos = true;
    cpu.host.timers.publish();
    // The screen: a DOS program has it from the start.
    if cpu.host.video.is_none() {
        cpu.host.video = super::video::Video::open(cpu.host.title, cpu.host.icon);
    }
}

fn set_al(cpu: &mut Cpu, value: u32) {
    cpu.eax = (cpu.eax & !0xff) | (value & 0xff);
}

fn set_ax(cpu: &mut Cpu, value: u32) {
    cpu.eax = (cpu.eax & !0xffff) | (value & 0xffff);
}

/// `int N` from the translated code.
pub fn interrupt(cpu: &mut Cpu, number: u8) {
    let ah = (cpu.eax >> 8) & 0xff;
    let ax = cpu.eax & 0xffff;
    if cpu.trace {
        println!("int {number:02x}h ax={ax:04x} bx={:04x} cx={:04x} dx={:04x}", cpu.ebx & 0xffff, cpu.ecx & 0xffff, cpu.edx & 0xffff);
    }
    match (number, ah, ax) {
        // DOS's version: 7.10, and no other extender's mark in the high half.
        (0x21, 0x30, _) => {
            cpu.eax = 0x0a07;
            cpu.ebx &= !0xffff;
        }
        // Resizing the program's memory block: there's room.
        (0x21, 0x4a, _) => cpu.cf = false,
        // DOS/4GW's own check: it's there.
        (0x21, _, 0xff00) => set_al(cpu, 0xff),
        // Is Windows running in enhanced mode? No: AL 0, plain DOS.
        (0x2f, _, 0x1600) => set_al(cpu, 0),
        // A Windows device driver's entry point, such as the joystick's or
        // the virtual sound card's: none, a null pointer in DI.
        (0x2f, _, 0x1684) => cpu.edi &= !0xffff,
        // Novell's IPX network driver, for multiplayer: not installed.
        (0x2f, _, 0x7a00) => set_al(cpu, 0),
        // Giving up the time slice to a multitasker: nothing to wait for.
        (0x2f, _, 0x1680) => {}
        // DPMI: a selector's base, in CX:DX.
        (0x31, _, 0x0006) => {
            let base = selector_base(cpu, cpu.ebx as u16);
            cpu.ecx = (cpu.ecx & !0xffff) | base >> 16;
            cpu.edx = (cpu.edx & !0xffff) | base & 0xffff;
            cpu.cf = false;
        }
        (0x21, _, _) if dos_call(cpu, ah) => {}
        (0x31, _, _) if dpmi_call(cpu, ax) => {}
        (0x10, _, _) if video_bios(cpu) => {}
        // The BIOS clock: ticks since midnight in CX:DX, about 18.2 a second,
        // as the BIOS data area holds them, and no midnight passed.
        (0x1a, 0x00, _) => {
            let ticks = cpu.r32(0x46c);
            cpu.ecx = (cpu.ecx & !0xffff) | ticks >> 16;
            cpu.edx = (cpu.edx & !0xffff) | ticks & 0xffff;
            set_al(cpu, 0);
        }
        (0x33, _, _) if mouse_call(cpu, ax) => {}
        (0x2f, 0x15, _) if cd_call(cpu) => {}
        _ => panic!("the runtime doesn't have int {number:02x}h with ah={ah:02x} (ax={ax:04x}) yet"),
    }
}
