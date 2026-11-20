//! A controller, working as a mouse. Strategy games like these have too many
//! buttons on screen to map to a controller's, so the left stick moves the
//! pointer, the right stick moves it slowly for small targets, and buttons or
//! triggers click. The game only ever sees mouse messages and keys.
//!
//! For a DOS program it's also a joystick on the PC's game port: two sticks
//! of two axes and four buttons, as a 4-button joystick of the time was. The
//! left stick is the first stick, the right stick the second, and A, B, X
//! and Y are the buttons. A program times each axis by counting its reads of
//! the port after writing to it, and the runtime answers each count as a
//! real stick would, so the program's own calibration works.

use crate::cpu::Cpu;
use crate::host::video;
use std::time::Instant;

const WM_KEYDOWN: u32 = 0x100;
const WM_KEYUP: u32 = 0x101;
const WM_MOUSEMOVE: u32 = 0x200;
const WM_LBUTTONDOWN: u32 = 0x201;
const WM_LBUTTONUP: u32 = 0x202;
const WM_RBUTTONDOWN: u32 = 0x204;
const WM_RBUTTONUP: u32 = 0x205;
const MK_LBUTTON: u32 = 1;
const MK_RBUTTON: u32 = 2;

/// The game port's answer, in reads after a write: a centred stick's, and
/// how far a full tilt moves it either way. Reads run far faster than on a
/// PC of the time, so these are a period stick's counts, not its time.
const PORT_CENTRE: f32 = 500.0;
const PORT_SPAN: f32 = 400.0;
/// SDL's west and north buttons (X and Y on an Xbox pad).
const BUTTON_WEST: u8 = 2;
const BUTTON_NORTH: u8 = 3;
pub const AXIS_LEFT_TRIGGER: u8 = 4;
pub const AXIS_RIGHT_TRIGGER: u8 = 5;
const TRIGGER_PRESS: f32 = 0.5;
const TRIGGER_RELEASE: f32 = 0.3;
pub const BUTTON_SOUTH: u8 = 0;
pub const BUTTON_EAST: u8 = 1;
pub const BUTTON_START: u8 = 6;
const BUTTON_BACK: u8 = 4;
const BUTTON_RIGHT_SHOULDER: u8 = 10;
const BUTTON_DPAD_UP: u8 = 11;
const BUTTON_DPAD_DOWN: u8 = 12;
const BUTTON_DPAD_LEFT: u8 = 13;
const BUTTON_DPAD_RIGHT: u8 = 14;
const KEY_ESCAPE: u32 = 0x1b;
const KEY_LEFT: u32 = 0x25;
const KEY_UP: u32 = 0x26;
const KEY_RIGHT: u32 = 0x27;
const KEY_DOWN: u32 = 0x28;

const SCREEN_RIGHT: i32 = 639;
const SCREEN_BOTTOM: i32 = 479;
/// Worn sticks don't quite centre, so small readings count as none.
const DEAD_ZONE: f32 = 0.18;
/// Pixels a second at full tilt, for the left stick and the right stick.
const SPEEDS: [f32; 2] = [420.0, 90.0];
/// Longer gaps are a stall, which shouldn't throw the pointer across the screen.
const LONGEST_STEP: f32 = 0.1;

pub struct Pad {
    /// Each stick's x and y, from -1 to 1.
    pub sticks: [[f32; 2]; 2],
    /// Whether the left and right triggers are pressed.
    triggers: [bool; 2],
    last: Instant,
    /// Fractions of a pixel carried over, so slow movement still moves.
    leftover: [f32; 2],
    /// Mouse buttons the controller holds, as `MK_LBUTTON` and `MK_RBUTTON`.
    pub held: u32,
    /// The game port's buttons held: bit 0 is the first stick's first
    /// button, then its second, then the second stick's two.
    port_buttons: u32,
    /// Reads of the game port since the program last wrote to it.
    port_reads: u32,
    /// How far each trigger is pulled, from 0 to 1.
    pulled: [f32; 2],
}

impl Default for Pad {
    fn default() -> Pad {
        Pad { sticks: [[0.0; 2]; 2], triggers: [false; 2], last: Instant::now(), leftover: [0.0; 2], held: 0, port_buttons: 0, port_reads: 0, pulled: [0.0; 2] }
    }
}

/// A program's calibration for these counts, as `min@centre@max`.
pub fn port_calibration() -> String {
    format!("{}@{}@{}", (PORT_CENTRE - PORT_SPAN) as u32, PORT_CENTRE as u32, (PORT_CENTRE + PORT_SPAN) as u32)
}

/// A write to the game port starts timing the axes again.
pub fn game_port_write(cpu: &mut Cpu) {
    cpu.host.pad.port_reads = 0;
}

/// A read of the game port: bits 0 to 3 stay set while each axis is still
/// timing, the first stick's x and y and then the second's, and bits 4 to 7
/// are its buttons, clear while held.
pub fn game_port_read(cpu: &mut Cpu) -> u32 {
    let pad = &mut cpu.host.pad;
    let reads = pad.port_reads as f32;
    pad.port_reads = pad.port_reads.saturating_add(1);
    let mut value = !(pad.port_buttons << 4) & 0xf0;
    // The second stick's y is the throttle, from the triggers: the right
    // one pushes it forward, the left one back, and the right stick's y
    // adds to it.
    let throttle = pad.sticks[1][1] + pad.pulled[0] - pad.pulled[1];
    let axes = [pad.sticks[0][0], pad.sticks[0][1], pad.sticks[1][0], throttle];
    for (bit, position) in axes.into_iter().enumerate() {
        let count = PORT_CENTRE + position.clamp(-1.0, 1.0) * PORT_SPAN;
        if reads < count {
            value |= 1 << bit;
        }
    }
    value
}

/// Past the dead zone, a squared curve gives fine control near the centre.
fn curve(value: f32) -> f32 {
    let beyond = ((value.abs() - DEAD_ZONE) / (1.0 - DEAD_ZONE)).clamp(0.0, 1.0);
    value.signum() * beyond * beyond
}

/// Records an axis, numbered as SDL does: left x and y, right x and y, then the
/// left and right triggers.
pub fn set_axis(cpu: &mut Cpu, axis: u8, value: f32) {
    match axis {
        0..=3 => cpu.host.pad.sticks[(axis / 2) as usize][(axis % 2) as usize] = value,
        // For a DOS program the triggers are the joystick's throttle.
        AXIS_LEFT_TRIGGER | AXIS_RIGHT_TRIGGER if cpu.host.timers.dos => cpu.host.pad.pulled[(axis - AXIS_LEFT_TRIGGER) as usize] = value.clamp(0.0, 1.0),
        AXIS_LEFT_TRIGGER => set_trigger(cpu, 0, BUTTON_EAST, value),
        AXIS_RIGHT_TRIGGER => set_trigger(cpu, 1, BUTTON_SOUTH, value),
        _ => {}
    }
}

/// A trigger is a button with two thresholds, so a finger resting near one
/// doesn't click over and over.
fn set_trigger(cpu: &mut Cpu, index: usize, button: u8, value: f32) {
    let held = cpu.host.pad.triggers[index];
    let down = value >= if held { TRIGGER_RELEASE } else { TRIGGER_PRESS };
    if down != held {
        cpu.host.pad.triggers[index] = down;
        set_button(cpu, button, down);
    }
}

/// Moves the pointer for the time since the last call.
pub fn advance(cpu: &mut Cpu) {
    let pad = &mut cpu.host.pad;
    let now = Instant::now();
    let elapsed = (now - pad.last).as_secs_f32().min(LONGEST_STEP);
    pad.last = now;
    let mut whole = [0i32; 2];
    for axis in 0..2 {
        let wanted: f32 = (0..2).map(|stick| curve(pad.sticks[stick][axis]) * SPEEDS[stick] * elapsed).sum();
        let total = pad.leftover[axis] + wanted;
        whole[axis] = total.trunc() as i32;
        pad.leftover[axis] = total.fract();
    }
    if whole == [0, 0] {
        return;
    }
    let held = pad.held;
    let (x, y) = cpu.host.pointer;
    let x = (x as i32 + whole[0]).clamp(0, SCREEN_RIGHT) as u32;
    let y = (y as i32 + whole[1]).clamp(0, SCREEN_BOTTOM) as u32;
    cpu.host.pointer = (x, y);
    cpu.host.post((WM_MOUSEMOVE, held, x | (y << 16)));
}

/// Handles a controller button going down or up.
pub fn set_button(cpu: &mut Cpu, button: u8, down: bool) {
    if button == BUTTON_RIGHT_SHOULDER {
        if down {
            video::toggle_stats(cpu);
        }
        return;
    }
    // Back (Select, View) opens the game's menu, where Windows had a menu bar.
    if button == BUTTON_BACK {
        if !down {
            cpu.host.menu_wanted = true;
        }
        return;
    }
    let dos = cpu.host.timers.dos;
    if dos {
        // The game port's buttons; A and B click as well, below.
        let bit = match button {
            BUTTON_SOUTH => 1,
            BUTTON_EAST => 2,
            BUTTON_WEST => 4,
            BUTTON_NORTH => 8,
            _ => 0,
        };
        let pad = &mut cpu.host.pad;
        pad.port_buttons = if down { pad.port_buttons | bit } else { pad.port_buttons & !bit };
    }
    let (x, y) = cpu.host.pointer;
    let position = x | (y << 16);
    let mouse = |pressed, released, flag| (if down { pressed } else { released }, flag);
    let click = match button {
        BUTTON_SOUTH => Some(mouse(WM_LBUTTONDOWN, WM_LBUTTONUP, MK_LBUTTON)),
        BUTTON_EAST => Some(mouse(WM_RBUTTONDOWN, WM_RBUTTONUP, MK_RBUTTON)),
        _ => None,
    };
    if let Some((message, flag)) = click {
        let pad = &mut cpu.host.pad;
        pad.held = if down { pad.held | flag } else { pad.held & !flag };
        let held = pad.held;
        cpu.host.post((message, held, position));
        if dos {
            // A DOS program's mouse handler hears it, as from the mouse.
            super::dos::mouse_button_state(cpu, held);
        }
        return;
    }
    let key = match button {
        BUTTON_START => KEY_ESCAPE,
        BUTTON_DPAD_UP => KEY_UP,
        BUTTON_DPAD_DOWN => KEY_DOWN,
        BUTTON_DPAD_LEFT => KEY_LEFT,
        BUTTON_DPAD_RIGHT => KEY_RIGHT,
        _ => return,
    };
    if dos {
        // A DOS program reads scan codes from its keyboard interrupt.
        if let Some(code) = super::dinput::code_of_virtual_key(key) {
            super::pc::key(cpu, code, down);
        }
        return;
    }
    cpu.host.post((if down { WM_KEYDOWN } else { WM_KEYUP }, key, 0));
}
