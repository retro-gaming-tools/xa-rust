//! A dialog box's controls, drawn over the game as a list of rows.
//!
//! Games with a Windows menu bar keep their option screens as dialog boxes,
//! made of the system's buttons, tick boxes, sliders and lists. There are no
//! such controls here, and a controller couldn't work them, so each one
//! becomes a row of a panel, as in `menu`: a button to press, a box to tick,
//! a set of choices or a slider to move left and right, a line of a list.
//!
//! Up and down, the d-pad or the mouse choose a row; left and right change a
//! choice or a slider; Enter, Space, A or a click presses; Escape, B or the
//! right mouse button asks to leave. `host::dialog_box` decides what each of
//! those means to the game, and calls `step` once a frame, so the game's own
//! code can run between frames.

use super::*;

/// What a row is, and what it shows.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Text,
    Button,
    /// A box that is ticked or not.
    Check(bool),
    /// Several choices side by side, of which at most one is chosen.
    Choice { options: Vec<String>, chosen: Option<usize> },
    Slider { low: i32, high: i32, at: i32 },
    /// A line of a list, and whether it's the one selected in the list.
    Item { current: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub label: String,
    pub kind: Kind,
    /// Whether it can be chosen. Text and greyed controls can't.
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct View {
    pub title: String,
    pub rows: Vec<Row>,
}

/// What the player did in one frame, by row.
#[derive(Debug, PartialEq)]
pub enum Action {
    Press(usize),
    /// Left (-1) or right (1) on a choice or a slider.
    Step(usize, i32),
    /// A click on one of a row's choices.
    Choose(usize, usize),
    /// A click on a slider, at this value.
    Slide(usize, i32),
    Back,
}

/// What the panel keeps between frames.
#[derive(Default)]
pub struct Panel {
    pub selected: usize,
    placed: bool,
    keys: Vec<u32>,
    mouse: u32,
    pad: u32,
    pressed_row: Option<usize>,
    /// Releases of what was held when the panel opened, for the game.
    later: Vec<Event>,
}

const PADDING: f32 = 12.0;
const GLYPH: f32 = 8.0;
const TITLE_SCALE: f32 = 2.0;
const NARROWEST: f32 = 260.0;
const TALLEST_ROW: f32 = 18.0;
const LOWEST_ROW: f32 = 10.0;
const BAR: f32 = 160.0;
const VALUE_COLUMNS: f32 = 5.0;
const KEY_RETURN: u32 = 13;
const KEY_ESCAPE: u32 = 27;
const KEY_SPACE: u32 = 32;
const KEY_RIGHT: u32 = 0x4000_004f;
const KEY_LEFT: u32 = 0x4000_0050;
const KEY_DOWN: u32 = 0x4000_0051;
const KEY_UP: u32 = 0x4000_0052;
const KEY_KEYPAD_ENTER: u32 = 0x4000_0058;
const MOUSE_LEFT: u8 = 1;
const MOUSE_RIGHT: u8 = 3;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;

/// The debug font has only ASCII.
fn printable(text: &str) -> CString {
    CString::new(text.chars().map(|c| if c.is_ascii_graphic() || c == ' ' { c } else { '?' }).collect::<String>()).unwrap()
}

/// How many characters a row needs.
fn columns(row: &Row) -> usize {
    let label = row.label.chars().count();
    match &row.kind {
        Kind::Text | Kind::Button => label,
        Kind::Check(_) | Kind::Item { .. } => label + 4,
        Kind::Choice { options, .. } => label + 2 + options.iter().map(|option| option.chars().count() + 2).sum::<usize>(),
        Kind::Slider { .. } => label + 2 + ((BAR / GLYPH) + VALUE_COLUMNS) as usize,
    }
}

pub struct Layout {
    panel: [f32; 4],
    title: (f32, f32, CString),
    row_height: f32,
    rows: usize,
}

pub fn layout(view: &View) -> Layout {
    let title_height = GLYPH * TITLE_SCALE;
    let frame = PADDING + title_height + PADDING + PADDING;
    let count = view.rows.len().max(1) as f32;
    let row_height = ((HEIGHT as f32 - 8.0 - frame) / count).floor().clamp(LOWEST_ROW, TALLEST_ROW);
    let widest = view.rows.iter().map(columns).max().unwrap_or(0) as f32 * GLYPH + 20.0;
    let title_width = view.title.chars().count() as f32 * GLYPH * TITLE_SCALE;
    let width = (widest.max(title_width) + 2.0 * PADDING).max(NARROWEST).min(WIDTH as f32 - 16.0);
    let height = frame + count * row_height;
    let left = ((WIDTH as f32 - width) / 2.0).floor();
    let top = ((HEIGHT as f32 - height) / 2.0).max(4.0).floor();
    Layout {
        panel: [left, top, width, height],
        title: (left + (width - title_width) / 2.0, top + PADDING, printable(&view.title)),
        row_height,
        rows: view.rows.len(),
    }
}

impl Layout {
    fn row_rect(&self, index: usize) -> [f32; 4] {
        let [left, top, width, _] = self.panel;
        [left + 4.0, top + PADDING + GLYPH * TITLE_SCALE + PADDING + index as f32 * self.row_height, width - 8.0, self.row_height]
    }

    fn row_at(&self, view: &View, x: f32, y: f32) -> Option<usize> {
        (0..self.rows).find(|index| {
            let [left, top, width, height] = self.row_rect(*index);
            view.rows[*index].active && x >= left && x < left + width && y >= top && y < top + height
        })
    }

    /// Where a row's choices are: the left edge and width of each.
    fn choices(&self, index: usize, options: &[String]) -> Vec<(f32, f32)> {
        let rect = self.row_rect(index);
        let widths: Vec<f32> = options.iter().map(|option| (option.chars().count() + 2) as f32 * GLYPH).collect();
        let mut x = rect[0] + rect[2] - 6.0 - widths.iter().sum::<f32>();
        widths
            .iter()
            .map(|width| {
                x += width;
                (x - width, *width)
            })
            .collect()
    }

    /// Where a row's slider bar is: its left edge.
    fn bar(&self, index: usize) -> f32 {
        let rect = self.row_rect(index);
        rect[0] + rect[2] - 6.0 - VALUE_COLUMNS * GLYPH - BAR
    }
}

/// The next row that can be chosen, `step` rows on, going round.
fn next_row(view: &View, from: usize, step: i32) -> usize {
    let count = view.rows.len() as i32;
    let mut at = from as i32;
    for _ in 0..count {
        at = (at + step).rem_euclid(count);
        if view.rows[at as usize].active {
            return at as usize;
        }
    }
    from
}

/// What a click at `x` on a row means.
fn click(view: &View, layout: &Layout, index: usize, x: f32) -> Option<Action> {
    match &view.rows[index].kind {
        Kind::Choice { options, .. } => {
            layout.choices(index, options).iter().position(|(left, width)| x >= *left && x < left + width).map(|option| Action::Choose(index, option))
        }
        Kind::Slider { low, high, .. } => {
            let share = ((x - layout.bar(index)) / BAR).clamp(0.0, 1.0);
            Some(Action::Slide(index, low + (share * (high - low) as f32).round() as i32))
        }
        Kind::Text => None,
        _ => Some(Action::Press(index)),
    }
}

impl Video {
    fn draw_controls(&self, view: &View, layout: &Layout, selected: usize) {
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
            for (index, row) in view.rows.iter().enumerate() {
                let rect = layout.row_rect(index);
                let text_y = (rect[1] + (layout.row_height - GLYPH) / 2.0).floor();
                let chosen = index == selected && row.active;
                // The colour of words on this row, and of the one choice made in it.
                let (ink, lit): ((u8, u8, u8), (u8, u8, u8)) = if chosen {
                    (self.set_color)(r, 230, 180, 70, 255);
                    (self.fill_rect)(r, rect.as_ptr());
                    ((20, 20, 20), (20, 20, 20))
                } else if row.active {
                    ((230, 230, 230), (255, 220, 120))
                } else if row.kind == Kind::Text {
                    ((190, 196, 210), (190, 196, 210))
                } else {
                    ((110, 116, 130), (110, 116, 130))
                };
                (self.set_color)(r, ink.0, ink.1, ink.2, 255);
                let label = match &row.kind {
                    Kind::Check(true) => format!("[x] {}", row.label),
                    Kind::Check(false) => format!("[ ] {}", row.label),
                    Kind::Item { current: true } => format!("> {}", row.label),
                    Kind::Item { current: false } => format!("  {}", row.label),
                    _ => row.label.clone(),
                };
                (self.debug_text)(r, rect[0] + 6.0, text_y, printable(&label).as_ptr());
                match &row.kind {
                    Kind::Choice { options, chosen: made } => {
                        for (option, (left, width)) in layout.choices(index, options).iter().enumerate() {
                            if *made == Some(option) {
                                // A box round the choice made, in the row's ink.
                                (self.set_color)(r, lit.0, lit.1, lit.2, 255);
                                let (top, bottom, right) = (rect[1] + 1.0, rect[1] + layout.row_height - 2.0, left + width - 1.0);
                                for (x1, y1, x2, y2) in [(*left, top, right, top), (*left, bottom, right, bottom), (*left, top, *left, bottom), (right, top, right, bottom)] {
                                    (self.draw_line)(r, x1, y1, x2, y2);
                                }
                            } else {
                                (self.set_color)(r, ink.0, ink.1, ink.2, 255);
                            }
                            (self.debug_text)(r, left + GLYPH, text_y, printable(&options[option]).as_ptr());
                        }
                    }
                    Kind::Slider { low, high, at } => {
                        let left = layout.bar(index);
                        let middle = rect[1] + layout.row_height / 2.0;
                        (self.draw_line)(r, left, middle, left + BAR, middle);
                        let share = if high > low { (at - low) as f32 / (high - low) as f32 } else { 0.0 };
                        let knob = [left + (share * BAR).round() - 2.0, rect[1] + 2.0, 5.0, layout.row_height - 4.0];
                        (self.set_color)(r, lit.0, lit.1, lit.2, 255);
                        (self.fill_rect)(r, knob.as_ptr());
                        let value = printable(&at.to_string());
                        (self.debug_text)(r, left + BAR + GLYPH, text_y, value.as_ptr());
                    }
                    _ => {}
                }
            }
            (self.set_blend)(r, BLEND_NONE);
            if self.gl.is_none() {
                (self.render_present)(r);
            }
        }
    }
}

/// Shows the system's pointer for the panel: the game draws its own and
/// hides the system's.
pub fn open(cpu: &mut Cpu) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    unsafe {
        (video.set_relative)(video.window, false);
        (video.show_cursor)();
    }
    video.relative = false;
}

/// Hands the game's frame back, and the game the releases it's owed.
pub fn close(cpu: &mut Cpu, panel: &mut Panel) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    unsafe { (video.hide_cursor)() };
    video.shown_frame = None;
    for event in std::mem::take(&mut panel.later) {
        handle_event(cpu, &event);
    }
}

/// One frame of the panel: takes the player's input, draws the rows, and
/// returns what the player did.
pub fn step(cpu: &mut Cpu, view: &View, panel: &mut Panel) -> Vec<Action> {
    let mut actions = Vec::new();
    let Some(mut video) = cpu.host.video.take() else { return actions };
    let shown = layout(view);
    if !panel.placed || !view.rows.get(panel.selected).is_some_and(|row| row.active) {
        panel.selected = (0..view.rows.len()).find(|index| view.rows[*index].active).unwrap_or(0);
        panel.placed = true;
    }
    let sideways = |row: usize| matches!(view.rows.get(row).map(|row| &row.kind), Some(Kind::Choice { .. } | Kind::Slider { .. }));
    let mut event = Event([0; 128]);
    while unsafe { (video.poll_event)(&mut event) } {
        video.convert(&mut event);
        match event.u32_at(0) {
            EVENT_QUIT => std::process::exit(0),
            EVENT_GAMEPAD_ADDED => unsafe {
                (video.open_gamepad)(event.u32_at(16));
            },
            EVENT_MOUSE_MOTION => {
                if let Some(row) = shown.row_at(view, event.f32_at(28), event.f32_at(32)) {
                    panel.selected = row;
                }
            }
            EVENT_MOUSE_BUTTON_DOWN => {
                panel.mouse |= 1 << (event.0[24] & 31);
                if event.0[24] == MOUSE_LEFT {
                    panel.pressed_row = shown.row_at(view, event.f32_at(28), event.f32_at(32));
                    panel.selected = panel.pressed_row.unwrap_or(panel.selected);
                }
            }
            EVENT_MOUSE_BUTTON_UP => {
                let bit = 1 << (event.0[24] & 31);
                if panel.mouse & bit == 0 {
                    panel.later.push(Event(event.0));
                    continue;
                }
                panel.mouse &= !bit;
                match event.0[24] {
                    MOUSE_LEFT => {
                        let on = shown.row_at(view, event.f32_at(28), event.f32_at(32));
                        if let Some(row) = on.filter(|_| on == panel.pressed_row) {
                            actions.extend(click(view, &shown, row, event.f32_at(28)));
                        }
                        panel.pressed_row = None;
                    }
                    MOUSE_RIGHT => actions.push(Action::Back),
                    _ => {}
                }
            }
            EVENT_KEY_DOWN => {
                let key = event.u32_at(28);
                if key == KEY_F10 || key == KEY_F11 {
                    cpu.host.video = Some(video);
                    handle_event(cpu, &event);
                    video = cpu.host.video.take().unwrap();
                    continue;
                }
                if !panel.keys.contains(&key) {
                    panel.keys.push(key);
                }
                match key {
                    KEY_UP => panel.selected = next_row(view, panel.selected, -1),
                    KEY_DOWN => panel.selected = next_row(view, panel.selected, 1),
                    KEY_LEFT if sideways(panel.selected) => actions.push(Action::Step(panel.selected, -1)),
                    KEY_RIGHT if sideways(panel.selected) => actions.push(Action::Step(panel.selected, 1)),
                    _ => {}
                }
            }
            EVENT_KEY_UP => {
                let key = event.u32_at(28);
                let Some(at) = panel.keys.iter().position(|k| *k == key) else {
                    panel.later.push(Event(event.0));
                    continue;
                };
                panel.keys.swap_remove(at);
                match key {
                    KEY_RETURN | KEY_KEYPAD_ENTER | KEY_SPACE => actions.push(Action::Press(panel.selected)),
                    KEY_ESCAPE => actions.push(Action::Back),
                    _ => {}
                }
            }
            EVENT_GAMEPAD_BUTTON_DOWN => {
                panel.pad |= 1 << (event.0[20] & 31);
                match event.0[20] {
                    BUTTON_DPAD_UP => panel.selected = next_row(view, panel.selected, -1),
                    BUTTON_DPAD_DOWN => panel.selected = next_row(view, panel.selected, 1),
                    BUTTON_DPAD_LEFT if sideways(panel.selected) => actions.push(Action::Step(panel.selected, -1)),
                    BUTTON_DPAD_RIGHT if sideways(panel.selected) => actions.push(Action::Step(panel.selected, 1)),
                    _ => {}
                }
            }
            EVENT_GAMEPAD_BUTTON_UP => {
                let bit = 1 << (event.0[20] & 31);
                if panel.pad & bit == 0 {
                    panel.later.push(Event(event.0));
                    continue;
                }
                panel.pad &= !bit;
                match event.0[20] {
                    gamepad::BUTTON_SOUTH => actions.push(Action::Press(panel.selected)),
                    gamepad::BUTTON_EAST | gamepad::BUTTON_START => actions.push(Action::Back),
                    _ => {}
                }
            }
            EVENT_GAMEPAD_REMOVED => panel.later.push(Event(event.0)),
            _ => {}
        }
    }
    // A press on words alone, or on a greyed row, means nothing.
    actions.retain(|action| match action {
        Action::Press(row) | Action::Step(row, _) | Action::Choose(row, _) | Action::Slide(row, _) => view.rows.get(*row).is_some_and(|row| row.active),
        Action::Back => true,
    });
    video.draw_controls(view, &shown, panel.selected);
    video.present_gl(Picture::Last, true);
    if !video.vsync {
        std::thread::sleep(REFRESH);
    }
    cpu.host.video = Some(video);
    actions
}

/// Draws the rows over the last frame and saves a picture of the window, for
/// scripted runs with a display.
pub fn picture(cpu: &mut Cpu, view: &View, name: &str) {
    let Some(video) = cpu.host.video.as_mut() else { return };
    let shown = layout(view);
    let first = (0..view.rows.len()).find(|index| view.rows[*index].active).unwrap_or(0);
    video.draw_controls(view, &shown, first);
    video.present_gl(Picture::Last, true);
    video.shown_frame = None;
    if let Some((width, height, pixels)) = video.window_picture() {
        let path = format!("{}/{name}.png", cpu.host.script.output);
        crate::host::script::write_png(&path, width, height, &pixels);
        println!("dialog saved as {name}.png");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view() -> View {
        let row = |label: &str, kind: Kind, active: bool| Row { label: label.to_string(), kind, active };
        View {
            title: "Options".to_string(),
            rows: vec![
                row("Speed", Kind::Choice { options: vec!["Low".into(), "Med".into(), "High".into()], chosen: Some(1) }, true),
                row("A note", Kind::Text, false),
                row("Detail", Kind::Slider { low: 0, high: 10, at: 5 }, true),
                row("OK", Kind::Button, true),
            ],
        }
    }

    #[test]
    fn the_panel_fits_the_screen_however_many_rows() {
        let mut many = view();
        for _ in 0..40 {
            many.rows.push(Row { label: "A line".to_string(), kind: Kind::Item { current: false }, active: true });
        }
        for view in [view(), many] {
            let layout = layout(&view);
            let [x, y, w, h] = layout.panel;
            assert!(x >= 0.0 && y >= 0.0 && x + w <= WIDTH as f32);
            let last = layout.row_rect(view.rows.len() - 1);
            assert!(last[1] + last[3] <= y + h);
        }
    }

    #[test]
    fn moving_skips_rows_that_cannot_be_chosen() {
        let view = view();
        assert_eq!(next_row(&view, 0, 1), 2);
        assert_eq!(next_row(&view, 0, -1), 3);
    }

    #[test]
    fn a_click_finds_the_choice_and_the_slider_value() {
        let view = view();
        let layout = layout(&view);
        let Kind::Choice { options, .. } = &view.rows[0].kind else { unreachable!() };
        let places = layout.choices(0, options);
        assert_eq!(click(&view, &layout, 0, places[2].0 + 1.0), Some(Action::Choose(0, 2)));
        assert_eq!(click(&view, &layout, 0, layout.row_rect(0)[0] + 1.0), None);
        assert_eq!(click(&view, &layout, 2, layout.bar(2)), Some(Action::Slide(2, 0)));
        assert_eq!(click(&view, &layout, 2, layout.bar(2) + BAR), Some(Action::Slide(2, 10)));
        assert_eq!(click(&view, &layout, 3, 0.0), Some(Action::Press(3)));
        assert_eq!(click(&view, &layout, 1, 0.0), None);
    }
}
