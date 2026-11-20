//! Hardware ports, for code that was brought over from DOS and still talks to
//! the PC's hardware with `in` and `out`.
//!
//! Only what a game is seen to use is here. A port nobody answers reads as all
//! ones, as an empty bus does, and is named once in the log.

use crate::cpu::Cpu;
use std::time::Duration;

/// The VGA's status register. Bit 3 is set while the picture's beam travels
/// back to the top, and games wait for it to time their frames.
const VGA_STATUS: u16 = 0x3da;
const VERTICAL_RETRACE: u32 = 8;
const GAME_PORT: u16 = 0x201;
const DISPLAY_ENABLE: u32 = 1;
/// A VGA drew 640x480 sixty times a second, and the way back took about a
/// twentieth of that.
const REFRESH: Duration = Duration::from_micros(16_667);
const RETRACE: Duration = Duration::from_micros(800);
/// The longest one read sleeps. A game waiting for the retrace reads the port
/// in a loop, and would otherwise keep a processor busy doing so.
const LONGEST_SLEEP: Duration = Duration::from_millis(1);

fn log_once(port: u16, action: &str) {
    static SEEN: std::sync::Mutex<Vec<(u16, bool)>> = std::sync::Mutex::new(Vec::new());
    let key = (port, action == "read");
    let mut seen = SEEN.lock().unwrap();
    if !seen.contains(&key) {
        seen.push(key);
        println!("port {port:#x} was {action}, and the runtime has nothing there");
    }
}

pub fn read(cpu: &mut Cpu, port: u16, size: u32) -> u32 {
    if let Some(value) = super::pc::read(cpu, port).or_else(|| super::vga::read(cpu, port)) {
        return value;
    }
    match port {
        VGA_STATUS => {
            let period = REFRESH.as_micros() as u64;
            let phase = Duration::from_micros(cpu.host.started.elapsed().as_micros() as u64 % period);
            let in_retrace = phase >= REFRESH - RETRACE;
            // Sleep towards the next change, so the waiting loop costs nothing.
            let until_change = if in_retrace { REFRESH - phase } else { REFRESH - RETRACE - phase };
            let before = cpu.host.waited;
            super::sleep(cpu, until_change.min(LONGEST_SLEEP));
            cpu.host.retrace.0 += 1;
            cpu.host.retrace.1 += cpu.host.waited - before;
            if in_retrace { VERTICAL_RETRACE | DISPLAY_ENABLE } else { 0 }
        }
        // The game port: the controller, as a joystick (see `gamepad`).
        GAME_PORT => super::gamepad::game_port_read(cpu),
        _ => {
            log_once(port, "read");
            u32::MAX >> (32 - size * 8)
        }
    }
}

pub fn write(cpu: &mut Cpu, port: u16, _size: u32, value: u32) {
    if super::pc::write(cpu, port, value) || super::vga::write(cpu, port, value) {
        return;
    }
    if port == GAME_PORT {
        super::gamepad::game_port_write(cpu);
        return;
    }
    log_once(port, "written");
}
