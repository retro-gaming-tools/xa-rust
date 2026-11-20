//! Glide: the loader every Glide stub calls, `XA_3DFX.EXE` 1.23 at 0x10c3b60.
//!
//! The program calls each Glide function through a stub of its own, a 5-byte
//! `call` to this loader, one after another. On first use the original loaded
//! `GLIDE2X.OVL`, the Voodoo driver, through DPMI, looked up the function's
//! name in it, and rewrote the stub into a jump there. The names are in a
//! table of pointers, in the stubs' order, in upper case, such as
//! `_GRDRAWTRIANGLE@12`.
//!
//! Here the runtime's Glide does the work instead, on every call: the stub
//! names the function, and the call goes on as if the program had called the
//! function itself, with its arguments and return address. Where the stubs
//! and the names are is found once, in memory, so the same code serves every
//! version of the program.

use recomp_runtime::host::glide;
use recomp_runtime::Cpu;
use std::collections::HashMap;
use std::sync::Mutex;

const STUB_SIZE: u32 = 5;
const CALL: u8 = 0xe8;
/// The first name in the table, which no other string starts with.
const FIRST_NAME: &[u8] = b"_GRAADRAWLINE@8\0";

/// For each loader: its first stub and its table of names.
static FOUND: Mutex<Option<HashMap<u32, (u32, u32)>>> = Mutex::new(None);

pub fn f_010c3b60(cpu: &mut Cpu) {
    call_glide(cpu, 0x010c3b60);
}

/// The same loader in `XA_3DFX.EXE` E 1.1.0, from the English CD.
pub fn f_020c1fb0(cpu: &mut Cpu) {
    call_glide(cpu, 0x020c1fb0);
}

/// Whether the 5 bytes at `at` are a `call` to `loader`.
fn calls(cpu: &Cpu, at: u32, loader: u32) -> bool {
    cpu.r8(at) == CALL && at.wrapping_add(STUB_SIZE).wrapping_add(cpu.r32(at + 1)) == loader
}

/// The first stub and the name table that go with `loader`, found from a stub.
fn find(cpu: &Cpu, loader: u32, stub: u32) -> (u32, u32) {
    let mut first = stub;
    while calls(cpu, first - STUB_SIZE, loader) {
        first -= STUB_SIZE;
    }
    // The name table points at the first name, which sits in the program's data.
    let start = stub & 0xff00_0000;
    let end = start + 0x0100_0000;
    let mut name = None;
    let mut at = start;
    while at < end - FIRST_NAME.len() as u32 && name.is_none() {
        if cpu.r8(at) == FIRST_NAME[0] && (0..FIRST_NAME.len()).all(|i| cpu.r8(at + i as u32) == FIRST_NAME[i]) {
            name = Some(at);
        }
        at += 1;
    }
    let name = name.expect("Glide's names weren't found in the program");
    let mut table = start;
    while cpu.r32(table) != name {
        table += 4;
        assert!(table < end, "Glide's table of names wasn't found in the program");
    }
    (first, table)
}

/// The Glide program's frame limiter. Its main loop (0x201ff49 in 1.1) waits
/// until its 60-a-second clock (`0x220c120` in 1.1, set by the timer callback
/// 0x201ea86) has moved on by a number of ticks this setting chooses: 0, the
/// default, waits for 2 (30 frames a second, which the player measured on a
/// card with the GPU 8% busy), 1 for 1 (60), and 2 doesn't wait. Only a
/// debug key cycles it (F with Ctrl or Alt, with cheats on). The game moves
/// everything by the ticks a frame took, so a faster frame rate changes
/// nothing else; the software program isn't held to 30 and ran at 58.
/// Raised from 0 to 1 at each swap, so the debug key's 2 still works.
fn lift_frame_limit(cpu: &mut Cpu, loader: u32) {
    let setting = match loader {
        0x010c3b60 => 0x0117_349c_u32.wrapping_add(cpu.bases[3]),
        0x020c1fb0 => 0x0217_33bc,
        _ => return,
    };
    if cpu.r32(setting) == 0 {
        cpu.w32(setting, 1);
    }
}

fn call_glide(cpu: &mut Cpu, loader: u32) {
    let stub = cpu.pop() - STUB_SIZE;
    let known = FOUND.lock().unwrap().get_or_insert_with(HashMap::new).get(&loader).copied();
    let (first, table) = known.unwrap_or_else(|| {
        let found = find(cpu, loader, stub);
        FOUND.lock().unwrap().as_mut().unwrap().insert(loader, found);
        found
    });
    let index = (stub - first) / STUB_SIZE;
    let name = cpu.string(cpu.r32(table + index * 4));
    if name == "_GRBUFFERSWAP@4" {
        lift_frame_limit(cpu, loader);
    }
    match glide::find_ignoring_case(&name) {
        Some(function) => {
            if cpu.trace {
                let arguments: Vec<String> = (1..=6).map(|i| format!("{:x}", cpu.r32(cpu.esp + i * 4))).collect();
                println!("glide {name} {}", arguments.join(" "));
            }
            function(cpu)
        }
        None => panic!("the runtime doesn't have Glide's {name} yet"),
    }
}
