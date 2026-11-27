//! A game's Windows menu bar, drawn over the game as a list.
//!
//! Some games of the time had no menu screen of their own: the window had an
//! ordinary menu bar, with entries such as New Game and Options. The window
//! here has no menu bar, and a controller couldn't work one anyway, so the
//! menu the game loaded is shown as a panel instead. A row with an arrow opens
//! the rows below it, and the first row of such a list goes back. The game
//! greys rows out, ticks them and renames them as it would on a menu bar.
//!
//! It opens by itself where the game waits for a command, such as Z's title
//! picture, and anywhere with a tap of Alt or the controller's Back button.
//! Up and down, the d-pad or the mouse choose; Enter, Space, A or a click
//! picks; Escape, B or the right mouse button goes back, and closes the menu
//! at the top. Like the question in `dialog`, it runs its own loop, answers on
//! release, and only to presses made while it was open.

use super::*;

/// One entry of a menu: a command, a list of further entries, or a line.
#[derive(Clone, Default)]
pub struct Item {
    pub text: String,
    /// The key the menu names beside it, such as F2.
    pub shortcut: String,
    /// What the game is sent when it's picked. Lists and lines have none.
    pub command: u32,
    pub children: Vec<Item>,
    pub separator: bool,
    pub disabled: bool,
    pub checked: bool,
}

impl Item {
    /// The entry with this command, here or further down.
    pub fn find(&mut self, command: u32) -> Option<&mut Item> {
        if self.command == command && self.children.is_empty() && !self.separator {
            return Some(self);
        }
        self.children.iter_mut().find_map(|child| child.find(command))
    }
}

const ROW: f32 = 18.0;
const PADDING: f32 = 12.0;
const GLYPH: f32 = 8.0;
const TITLE_SCALE: f32 = 2.0;
const NARROWEST: f32 = 240.0;
const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_SPACE: u32 = 32;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const KEY_KEYPAD_ENTER: u32 = 0x4000_0058;
const MOUSE_LEFT: u8 = 1;
const MOUSE_RIGHT: u8 = 3;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;

/// A row as drawn: its words, and which entry of the list it stands for.
/// `None` is the row that goes back.
struct Row {
    text: CString,
    right: CString,
    entry: Option<usize>,
    active: bool,
    separator: bool,
}

struct Layout {
    panel: [f32; 4],
    title: (f32, f32, CString),
    rows: Vec<Row>,
}

/// The debug font has only ASCII.
fn printable(text: &str) -> CString {
    CString::new(text.chars().map(|c| if c.is_ascii_graphic() || c == ' ' { c } else { '?' }).collect::<String>()).unwrap()
}

fn layout(title: &str, items: &[Item], nested: bool) -> Layout {
    let mut rows = Vec::new();
    if nested {
        rows.push(Row { text: printable("< Back"), right: printable(""), entry: None, active: true, separator: false });
    }
    for (index, item) in items.iter().enumerate() {
        // A command with no text shows as nothing on a menu bar, so it gets no
        // row here either, until the game gives it a name.
        if item.text.is_empty() && !item.separator && item.children.is_empty() {
            continue;
        }
        let mark = if item.checked { "* " } else { "" };
        let right = if item.children.is_empty() { item.shortcut.clone() } else { ">".to_string() };
        rows.push(Row {
            text: printable(&format!("{mark}{}", item.text)),
            right: printable(&right),
            entry: Some(index),
            active: !item.separator && !item.disabled,
            separator: item.separator,
        });
    }
    let widest = rows.iter().map(|row| row.text.as_bytes().len() + row.right.as_bytes().len() + 3).max().unwrap_or(0) as f32 * GLYPH;
    let title_width = title.chars().count() as f32 * GLYPH * TITLE_SCALE;
    let width = (widest.max(title_width) + 2.0 * PADDING).max(NARROWEST).min(WIDTH as f32 - 16.0);
    let title_height = GLYPH * TITLE_SCALE;
    let height = PADDING + title_height + PADDING + rows.len() as f32 * ROW + PADDING;
    let left = ((WIDTH as f32 - width) / 2.0).floor();
    let top = ((HEIGHT as f32 - height) / 2.0).max(4.0).floor();
    Layout { panel: [left, top, width, height], title: (left + (width - title_width) / 2.0, top + PADDING, printable(title)), rows }
}

impl Layout {
    fn row_rect(&self, index: usize) -> [f32; 4] {
        let [left, top, width, _] = self.panel;
        [left + 4.0, top + PADDING + GLYPH * TITLE_SCALE + PADDING + index as f32 * ROW, width - 8.0, ROW]
    }

    fn row_at(&self, x: f32, y: f32) -> Option<usize> {
        (0..self.rows.len()).find(|index| {
            let [left, top, width, height] = self.row_rect(*index);
            self.rows[*index].active && x >= left && x < left + width && y >= top && y < top + height
        })
    }

    /// The next row that can be picked, `step` rows on, going round.
    fn step(&self, from: usize, step: i32) -> usize {
        let count = self.rows.len() as i32;
        let mut at = from as i32;
        for _ in 0..count {
            at = (at + step).rem_euclid(count);
            if self.rows[at as usize].active {
                return at as usize;
            }
        }
        from
    }

    fn first(&self) -> usize {
        // Past the row that goes back, when there's anything else to pick.
        let start = usize::from(self.rows.first().is_some_and(|row| row.entry.is_none()));
        (start..self.rows.len()).find(|index| self.rows[*index].active).unwrap_or(0)
    }
}

impl Video {
    fn draw_menu(&self, layout: &Layout, selected: usize) {
        unsafe {
            let r = self.renderer;
            if self.gl.is_some() {
                self.clear_overlay();
            } else {
                (self.set_color)(r, 0, 0, 0, 255);
                (self.render_clear)(r);
                (self.render_texture)(r, self.texture, std::ptr::null(), std::ptr::null());
            }
            (self.set_blend)(r, BLEND_ALPHA);
            (self.set_color)(r, 0, 0, 0, 110);
            let whole = [0.0, 0.0, WIDTH as f32, HEIGHT as f32];
            (self.fill_rect)(r, whole.as_ptr());
            (self.set_color)(r, 20, 24, 32, 235);
            (self.fill_rect)(r, layout.panel.as_ptr());
            let [x, y, w, h] = layout.panel;
            (self.set_color)(r, 150, 160, 180, 255);
            for (x1, y1, x2, y2) in [(x, y, x + w - 1.0, y), (x, y + h - 1.0, x + w - 1.0, y + h - 1.0), (x, y, x, y + h - 1.0), (x + w - 1.0, y, x + w - 1.0, y + h - 1.0)] {
                (self.draw_line)(r, x1, y1, x2, y2);
            }
            (self.set_color)(r, 255, 220, 120, 255);
            (self.set_scale)(r, TITLE_SCALE, TITLE_SCALE);
            (self.debug_text)(r, (layout.title.0 / TITLE_SCALE).floor(), layout.title.1 / TITLE_SCALE, layout.title.2.as_ptr());
            (self.set_scale)(r, 1.0, 1.0);
            for (index, row) in layout.rows.iter().enumerate() {
                let rect = layout.row_rect(index);
                let text_y = (rect[1] + (ROW - GLYPH) / 2.0).floor();
                if row.separator {
                    (self.set_color)(r, 70, 78, 96, 255);
                    (self.draw_line)(r, rect[0] + 4.0, rect[1] + ROW / 2.0, rect[0] + rect[2] - 5.0, rect[1] + ROW / 2.0);
                    continue;
                }
                let chosen = index == selected && row.active;
                if chosen {
                    (self.set_color)(r, 230, 180, 70, 255);
                    (self.fill_rect)(r, rect.as_ptr());
                    (self.set_color)(r, 20, 20, 20, 255);
                } else if row.active {
                    (self.set_color)(r, 230, 230, 230, 255);
                } else {
                    (self.set_color)(r, 110, 116, 130, 255);
                }
                (self.debug_text)(r, rect[0] + 6.0, text_y, row.text.as_ptr());
                let right = row.right.as_bytes().len() as f32 * GLYPH;
                (self.debug_text)(r, rect[0] + rect[2] - 6.0 - right, text_y, row.right.as_ptr());
            }
            (self.set_blend)(r, BLEND_NONE);
            if self.gl.is_none() {
                (self.render_present)(r);
            }
        }
    }
}

/// What a press that ended in the menu meant.
enum Press {
    Pick,
    Back,
}

/// Shows the menu until the player picks a command, which is returned, or
/// closes it.
pub(super) fn run(cpu: &mut Cpu, title: &str, items: &[Item]) -> Option<u32> {
    let mut video = cpu.host.video.take().unwrap();
    unsafe {
        // The game draws its own pointer and hides the system's.
        (video.set_relative)(video.window, false);
        (video.show_cursor)();
    }
    video.relative = false;
    // The lists opened so far, by the entry picked in each, and the row to
    // come back to.
    let mut path: Vec<(usize, usize)> = Vec::new();
    let list_at = |path: &[(usize, usize)]| path.iter().fold(items, |list, (entry, _)| &list[*entry].children);
    let name_at = |path: &[(usize, usize)]| match path.split_last() {
        Some(((entry, _), above)) => list_at(above)[*entry].text.clone(),
        None => title.to_string(),
    };
    let mut shown = layout(title, items, false);
    let mut selected = shown.first();
    let (mut keys, mut mouse, mut pad): (Vec<u32>, u32, u32) = (Vec::new(), 0, 0);
    let mut later: Vec<Event> = Vec::new();
    let mut pressed_row: Option<usize> = None;
    let picked = 'menu: loop {
        let mut press = None;
        let mut event = Event([0; 128]);
        while unsafe { (video.poll_event)(&mut event) } {
            video.convert(&mut event);
            match event.u32_at(0) {
                EVENT_QUIT => std::process::exit(0),
                EVENT_GAMEPAD_ADDED => unsafe {
                    (video.open_gamepad)(event.u32_at(16));
                },
                EVENT_MOUSE_MOTION => {
                    if let Some(row) = shown.row_at(event.f32_at(28), event.f32_at(32)) {
                        selected = row;
                    }
                }
                EVENT_MOUSE_BUTTON_DOWN => {
                    mouse |= 1 << (event.0[24] & 31);
                    if event.0[24] == MOUSE_LEFT {
                        pressed_row = shown.row_at(event.f32_at(28), event.f32_at(32));
                        selected = pressed_row.unwrap_or(selected);
                    }
                }
                EVENT_MOUSE_BUTTON_UP => {
                    let bit = 1 << (event.0[24] & 31);
                    if mouse & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    mouse &= !bit;
                    match event.0[24] {
                        MOUSE_LEFT => {
                            let on = shown.row_at(event.f32_at(28), event.f32_at(32));
                            if on.is_some() && on == pressed_row {
                                press = Some(Press::Pick);
                            }
                            pressed_row = None;
                        }
                        MOUSE_RIGHT => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_KEY_DOWN => {
                    let key = event.u32_at(28);
                    if crate::host::hotkeys::is_window_key(key) {
                        cpu.host.video = Some(video);
                        handle_event(cpu, &event);
                        video = cpu.host.video.take().unwrap();
                        continue;
                    }
                    if !keys.contains(&key) {
                        keys.push(key);
                    }
                    match key {
                        KEY_UP => selected = shown.step(selected, -1),
                        KEY_DOWN => selected = shown.step(selected, 1),
                        _ => {}
                    }
                }
                EVENT_KEY_UP => {
                    let key = event.u32_at(28);
                    let Some(at) = keys.iter().position(|k| *k == key) else {
                        later.push(Event(event.0));
                        continue;
                    };
                    keys.swap_remove(at);
                    match key {
                        KEY_RETURN | KEY_KEYPAD_ENTER | KEY_SPACE => press = Some(Press::Pick),
                        KEY_ESCAPE => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_DOWN => {
                    pad |= 1 << (event.0[20] & 31);
                    match event.0[20] {
                        BUTTON_DPAD_UP => selected = shown.step(selected, -1),
                        BUTTON_DPAD_DOWN => selected = shown.step(selected, 1),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_BUTTON_UP => {
                    let bit = 1 << (event.0[20] & 31);
                    if pad & bit == 0 {
                        later.push(Event(event.0));
                        continue;
                    }
                    pad &= !bit;
                    match event.0[20] {
                        gamepad::BUTTON_SOUTH => press = Some(Press::Pick),
                        gamepad::BUTTON_EAST | gamepad::BUTTON_START => press = Some(Press::Back),
                        _ => {}
                    }
                }
                EVENT_GAMEPAD_REMOVED => later.push(Event(event.0)),
                _ => {}
            }
            // A row that goes back is a way of saying so.
            if matches!(press, Some(Press::Pick)) && shown.rows.get(selected).is_some_and(|row| row.entry.is_none()) {
                press = Some(Press::Back);
            }
            match press.take() {
                Some(Press::Back) => {
                    let Some((_, row)) = path.pop() else { break 'menu None };
                    shown = layout(&name_at(&path), list_at(&path), !path.is_empty());
                    selected = row.min(shown.rows.len().saturating_sub(1));
                }
                Some(Press::Pick) => {
                    let Some(row) = shown.rows.get(selected).filter(|row| row.active) else { continue };
                    let Some(entry) = row.entry else { continue };
                    let item = &list_at(&path)[entry];
                    if item.children.is_empty() {
                        break 'menu Some(item.command);
                    }
                    path.push((entry, selected));
                    shown = layout(&name_at(&path), list_at(&path), true);
                    selected = shown.first();
                }
                None => {}
            }
        }
        video.draw_menu(&shown, selected);
        video.present_gl(Picture::Last, true);
        if !video.vsync {
            std::thread::sleep(REFRESH);
        }
        // Sound plays on under the menu, as it did under a Windows menu bar.
        crate::host::audio::pump(cpu);
    };
    unsafe { (video.hide_cursor)() };
    // The game's frame goes back on screen at its next swap or flip.
    video.shown_frame = None;
    cpu.host.video = Some(video);
    for event in &later {
        handle_event(cpu, event);
    }
    picked
}

/// Draws the menu's first list over the last frame and saves a picture of
/// the window, for scripted runs with a display.
pub(super) fn show_once(cpu: &mut Cpu, title: &str, items: &[Item]) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    let shown = layout(title, items, false);
    video.draw_menu(&shown, shown.first());
    video.present_gl(Picture::Last, true);
    video.shown_frame = None;
    if let Some((width, height, pixels)) = video.window_picture() {
        let path = format!("{}/menu.png", cpu.host.script.output);
        crate::host::script::write_png(&path, width, height, &pixels);
        println!("menu saved as menu.png");
    }
}
