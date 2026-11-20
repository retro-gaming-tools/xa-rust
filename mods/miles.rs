//! The Miles Sound System (AIL 3), linked into the game, played by the
//! runtime instead of by Miles' sound card drivers.
//!
//! Miles drove the card through real-mode drivers (`SBPRO2.MDI` and the
//! like), which the runtime can't run. So the functions the game calls to set
//! up the drivers, play its sound effects and play its XMIDI music are
//! replaced here: effects become the mixer's voices, playing the game's own
//! memory, and music goes to the runtime's MIDI synthesizer. Miles' timers
//! and its CD audio (`AIL_redbook_*`) still run as written, on the runtime's
//! timer interrupt and MSCDEX.
//!
//! Each function has two names: its address in `XA_3DFX.EXE` 1.23, and in E
//! 1.1.0 from the English CD. They're found by the debug string each of
//! Miles' API functions logs its name with, such as `AIL_start_sample(0x%X)`.
//! Miles' API is cdecl: arguments on the stack, which the caller removes.

use recomp_runtime::host::{audio, midi, xmidi};
use recomp_runtime::Cpu;
use std::collections::HashMap;
use std::sync::Mutex;

const INIT_SUCCESS: u32 = 0;
/// A sample's or sequence's status, as Miles numbers them.
const DONE: u32 = 2;
const PLAYING: u32 = 4;
const STOPPED: u32 = 8;
/// Miles' volumes and pans run from 0 to 127, and a pan of 64 is the middle.
const FULL: u32 = 127;
const MIDDLE: f32 = 64.0;
/// What `AIL_init_sample` gives a sample: 8-bit mono at 11025 a second, at
/// full volume in the middle, played once.
const DEFAULT_RATE: u32 = 11_025;
/// Room for a handle the game may look into: Miles' own are smaller.
const HANDLE_SIZE: u32 = 0x400;
const WAVE_PCM: u32 = 1;

#[derive(Clone, Copy)]
struct Sample {
    data: u32,
    size: u32,
    channels: u32,
    bits: u32,
    rate: u32,
    volume: u32,
    pan: u32,
    plays: u32,
    /// Started and not ended: playing until the mixer runs out.
    started: bool,
}

impl Default for Sample {
    fn default() -> Sample {
        Sample { data: 0, size: 0, channels: 1, bits: 8, rate: DEFAULT_RATE, volume: FULL, pan: 64, plays: 1, started: false }
    }
}

struct State {
    samples: HashMap<u32, Sample>,
    /// `AIL_set_digital_master_volume`.
    master: u32,
    sequence: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<T>(change: impl FnOnce(&mut State) -> T) -> T {
    let mut state = STATE.lock().unwrap();
    change(state.get_or_insert_with(|| State { samples: HashMap::new(), master: FULL, sequence: 0 }))
}

fn sample(handle: u32) -> Sample {
    with(|state| state.samples.get(&handle).copied().unwrap_or_default())
}

fn set_sample(handle: u32, sample: Sample) {
    with(|state| state.samples.insert(handle, sample));
}

fn trace(cpu: &Cpu, text: impl FnOnce() -> String) {
    if cpu.trace {
        println!("  miles {}", text());
    }
}

/// A handle: a block of the game's memory, so it's a real address.
fn new_handle(cpu: &mut Cpu) -> u32 {
    let handle = cpu.allocate(HANDLE_SIZE);
    cpu.fill(handle, HANDLE_SIZE, 0);
    handle
}

/// Gives a voice its sample's volume and pan, with the master volume.
fn apply_levels(cpu: &mut Cpu, handle: u32, sample: &Sample) {
    let master = with(|state| state.master);
    let volume = sample.volume.min(FULL) as f32 / FULL as f32 * master.min(FULL) as f32 / FULL as f32;
    let pan = ((sample.pan.min(FULL) as f32 - MIDDLE) / (FULL as f32 - MIDDLE)).clamp(-1.0, 1.0);
    audio::voice_level(cpu, handle, volume, pan);
}

fn done(cpu: &mut Cpu, result: u32) {
    cpu.finish(result, 0);
}

// Drivers.

/// `AIL_install_MDI_INI(MDI_DRIVER **)` and `AIL_install_DIG_INI(DIG_DRIVER **)`:
/// a driver handle, and success.
fn install_driver(cpu: &mut Cpu) {
    let out = cpu.arg(0);
    let driver = new_handle(cpu);
    cpu.w32(out, driver);
    trace(cpu, || format!("driver {driver:#x}"));
    done(cpu, INIT_SUCCESS);
}

/// `AIL_allocate_sample_handle(DIG_DRIVER *)`.
fn allocate_sample(cpu: &mut Cpu) {
    let handle = new_handle(cpu);
    set_sample(handle, Sample::default());
    done(cpu, handle);
}

/// `AIL_allocate_sequence_handle(MDI_DRIVER *)`.
fn allocate_sequence(cpu: &mut Cpu) {
    let handle = new_handle(cpu);
    with(|state| state.sequence = handle);
    done(cpu, handle);
}

/// `AIL_release_sample_handle(HSAMPLE)`.
fn release_sample(cpu: &mut Cpu) {
    let handle = cpu.arg(0);
    audio::voice_remove(cpu, handle);
    with(|state| state.samples.remove(&handle));
    done(cpu, 0);
}

/// `AIL_release_sequence_handle(HSEQUENCE)`.
fn release_sequence(cpu: &mut Cpu) {
    midi::song_end(cpu);
    done(cpu, 0);
}

/// Uninstalling a driver, setting the file prefix for MT-32 timbres: nothing to do.
fn nothing(cpu: &mut Cpu) {
    done(cpu, 0);
}

/// `AIL_set_digital_master_volume(DIG_DRIVER *, volume)`.
fn master_volume(cpu: &mut Cpu) {
    let volume = cpu.arg(1);
    with(|state| state.master = volume);
    let playing: Vec<(u32, Sample)> = with(|state| state.samples.iter().map(|(h, s)| (*h, *s)).collect());
    for (handle, sample) in playing {
        apply_levels(cpu, handle, &sample);
    }
    done(cpu, 0);
}

// Sound effects.

/// `AIL_init_sample(HSAMPLE)`: back to the defaults, stopped.
fn init_sample(cpu: &mut Cpu) {
    let handle = cpu.arg(0);
    audio::voice_stop(cpu, handle);
    set_sample(handle, Sample::default());
    done(cpu, 0);
}

/// The format and sound of a WAV file in the game's memory: channels, bits,
/// rate, and where its samples are and how many bytes.
fn read_wave(cpu: &Cpu, at: u32) -> Option<(u32, u32, u32, u32, u32)> {
    if cpu.bytes(at, 4) != b"RIFF" || cpu.bytes(at + 8, 4) != b"WAVE" {
        return None;
    }
    let end = at + 8 + cpu.r32(at + 4);
    let (mut chunk, mut format, mut data) = (at + 12, None, None);
    while chunk + 8 <= end {
        let size = cpu.r32(chunk + 4);
        match cpu.bytes(chunk, 4) {
            b"fmt " => format = Some((cpu.r16(chunk + 8) as u32, cpu.r16(chunk + 10) as u32, cpu.r32(chunk + 12), cpu.r16(chunk + 22) as u32)),
            b"data" => data = Some((chunk + 8, size.min(end.saturating_sub(chunk + 8)))),
            _ => {}
        }
        chunk += 8 + size + (size & 1);
    }
    let ((tag, channels, rate, bits), (start, size)) = (format?, data?);
    (tag == WAVE_PCM && matches!(bits, 8 | 16) && channels > 0).then_some((channels, bits, rate, start, size))
}

/// Points a playing sample at new sound: it plays on where it was when the
/// sound is the same, as the game sets the same sound again each time it
/// moves an effect, and starts over otherwise.
fn change_sound(cpu: &mut Cpu, handle: u32, old: &Sample, new: &Sample) {
    if new.started && (old.data, old.size) != (new.data, new.size) {
        audio::voice_set(cpu, handle, new.data, new.size, new.channels, new.bits, new.rate);
        apply_levels(cpu, handle, new);
        audio::voice_start(cpu, handle, new.plays);
    }
}

/// `AIL_set_sample_file(HSAMPLE, void *file_image, s32 block)`: 1 when it's a
/// WAV file Miles could play.
fn set_sample_file(cpu: &mut Cpu) {
    let (handle, image) = (cpu.arg(0), cpu.arg(1));
    let old = sample(handle);
    let Some((channels, bits, rate, data, size)) = read_wave(cpu, image) else {
        trace(cpu, || format!("{handle:#x} can't play the file at {image:#x}"));
        return done(cpu, 0);
    };
    let new = Sample { data, size, channels, bits, rate, ..old };
    set_sample(handle, new);
    change_sound(cpu, handle, &old, &new);
    done(cpu, 1);
}

/// `AIL_set_sample_address(HSAMPLE, void *start, u32 length)`: raw sound in
/// the sample's format.
fn set_sample_address(cpu: &mut Cpu) {
    let (handle, data, size) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
    let old = sample(handle);
    let new = Sample { data, size, ..old };
    set_sample(handle, new);
    change_sound(cpu, handle, &old, &new);
    done(cpu, 0);
}

/// `AIL_set_sample_playback_rate(HSAMPLE, s32 rate)`.
fn set_sample_rate(cpu: &mut Cpu) {
    let (handle, rate) = (cpu.arg(0), cpu.arg(1));
    set_sample(handle, Sample { rate, ..sample(handle) });
    audio::voice_rate(cpu, handle, rate);
    done(cpu, 0);
}

/// `AIL_set_sample_volume(HSAMPLE, s32 volume)` and `AIL_set_sample_pan`.
fn set_sample_level(cpu: &mut Cpu, pan: bool) {
    let (handle, value) = (cpu.arg(0), cpu.arg(1) & 0xffff);
    let mut changed = sample(handle);
    if pan {
        changed.pan = value;
    } else {
        changed.volume = value;
    }
    set_sample(handle, changed);
    apply_levels(cpu, handle, &changed);
    done(cpu, 0);
}

/// `AIL_set_sample_loop_count(HSAMPLE, s32 count)`: 0 plays for ever.
fn set_sample_loops(cpu: &mut Cpu) {
    let (handle, plays) = (cpu.arg(0), cpu.arg(1));
    set_sample(handle, Sample { plays, ..sample(handle) });
    done(cpu, 0);
}

/// `AIL_start_sample(HSAMPLE)`: plays from the start.
fn start_sample(cpu: &mut Cpu) {
    let handle = cpu.arg(0);
    let started = Sample { started: true, ..sample(handle) };
    set_sample(handle, started);
    audio::voice_set(cpu, handle, started.data, started.size, started.channels, started.bits, started.rate);
    apply_levels(cpu, handle, &started);
    audio::voice_start(cpu, handle, started.plays);
    trace(cpu, || format!("{handle:#x} plays {} bytes at {:#x}, {} a second, volume {} pan {}", started.size, started.data, started.rate, started.volume, started.pan));
    done(cpu, 0);
}

/// `AIL_end_sample(HSAMPLE)`: stops it for good.
fn end_sample(cpu: &mut Cpu) {
    let handle = cpu.arg(0);
    audio::voice_stop(cpu, handle);
    set_sample(handle, Sample { started: false, ..sample(handle) });
    done(cpu, 0);
}

/// `AIL_sample_status(HSAMPLE)`.
fn sample_status(cpu: &mut Cpu) {
    let handle = cpu.arg(0);
    let status = if sample(handle).started && audio::voice_playing(cpu, handle) { PLAYING } else { DONE };
    done(cpu, status);
}

// Music.

/// `AIL_init_sequence(HSEQUENCE, void *start, s32 sequence_num)`: 1 when the
/// song could be read.
fn init_sequence(cpu: &mut Cpu) {
    let (start, number) = (cpu.arg(1), cpu.arg(2));
    match xmidi::parse_at(cpu, start, number) {
        Some(song) => {
            trace(cpu, || format!("song {number} at {start:#x}: {} events, {:.0} seconds", song.events.len(), song.length as f64 / xmidi::TICKS_PER_SECOND));
            midi::song_load(cpu, song);
            done(cpu, 1);
        }
        None => {
            println!("  Miles: the song at {start:#x} can't be read");
            done(cpu, 0);
        }
    }
}

fn start_sequence(cpu: &mut Cpu) {
    midi::song_start(cpu);
    done(cpu, 0);
}

fn stop_sequence(cpu: &mut Cpu) {
    midi::song_pause(cpu, true);
    done(cpu, 0);
}

fn resume_sequence(cpu: &mut Cpu) {
    let (playing, ended) = midi::song_state(cpu);
    if !playing && !ended {
        midi::song_pause(cpu, false);
    }
    done(cpu, 0);
}

fn end_sequence(cpu: &mut Cpu) {
    midi::song_end(cpu);
    done(cpu, 0);
}

/// `AIL_set_sequence_volume(HSEQUENCE, s32 volume, s32 milliseconds)`.
fn set_sequence_volume(cpu: &mut Cpu) {
    let (volume, milliseconds) = (cpu.arg(1) as i32, cpu.arg(2) as i32);
    midi::song_volume(cpu, volume.clamp(0, FULL as i32) as f32 / FULL as f32, milliseconds.max(0) as u32);
    done(cpu, 0);
}

/// `AIL_set_sequence_loop_count(HSEQUENCE, s32 count)`: 0 plays for ever.
fn set_sequence_loops(cpu: &mut Cpu) {
    let plays = cpu.arg(1);
    midi::song_plays(cpu, plays);
    done(cpu, 0);
}

/// For `AIL_sequence_status`, should the game ask.
#[allow(dead_code)]
fn sequence_status(cpu: &mut Cpu) {
    let (playing, ended) = midi::song_state(cpu);
    done(cpu, if playing { PLAYING } else if ended { DONE } else { STOPPED });
}

// XA_3DFX.EXE 1.23.
pub fn f_010aaac0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_MDI_INI
pub fn f_010ad0c0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_DIG_INI
pub fn f_010aa800(cpu: &mut Cpu) { allocate_sequence(cpu) } // AIL_allocate_sequence_handle
pub fn f_010ace00(cpu: &mut Cpu) { allocate_sample(cpu) } // AIL_allocate_sample_handle
pub fn f_010acc30(cpu: &mut Cpu) { release_sample(cpu) } // AIL_release_sample_handle
pub fn f_010aa750(cpu: &mut Cpu) { release_sequence(cpu) } // AIL_release_sequence_handle
pub fn f_010acf00(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_DIG_driver
pub fn f_010aa900(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_MDI_driver
pub fn f_010a9880(cpu: &mut Cpu) { nothing(cpu) } // AIL_set_GTL_filename_prefix
pub fn f_010aa630(cpu: &mut Cpu) { init_sequence(cpu) } // AIL_init_sequence
pub fn f_010aa580(cpu: &mut Cpu) { start_sequence(cpu) } // AIL_start_sequence
pub fn f_010aa4d0(cpu: &mut Cpu) { stop_sequence(cpu) } // AIL_stop_sequence
pub fn f_010aa420(cpu: &mut Cpu) { resume_sequence(cpu) } // AIL_resume_sequence
pub fn f_010aa370(cpu: &mut Cpu) { end_sequence(cpu) } // AIL_end_sequence
pub fn f_010aa1d0(cpu: &mut Cpu) { set_sequence_volume(cpu) } // AIL_set_sequence_volume
pub fn f_010aa110(cpu: &mut Cpu) { set_sequence_loops(cpu) } // AIL_set_sequence_loop_count
pub fn f_010acb80(cpu: &mut Cpu) { init_sample(cpu) } // AIL_init_sample
pub fn f_010aca60(cpu: &mut Cpu) { set_sample_file(cpu) } // AIL_set_sample_file
pub fn f_010ac990(cpu: &mut Cpu) { set_sample_address(cpu) } // AIL_set_sample_address
pub fn f_010ac540(cpu: &mut Cpu) { set_sample_rate(cpu) } // AIL_set_sample_playback_rate
pub fn f_010ac480(cpu: &mut Cpu) { set_sample_level(cpu, false) } // AIL_set_sample_volume
pub fn f_010ac3c0(cpu: &mut Cpu) { set_sample_level(cpu, true) } // AIL_set_sample_pan
pub fn f_010ac300(cpu: &mut Cpu) { set_sample_loops(cpu) } // AIL_set_sample_loop_count
pub fn f_010ac810(cpu: &mut Cpu) { start_sample(cpu) } // AIL_start_sample
pub fn f_010ac600(cpu: &mut Cpu) { end_sample(cpu) } // AIL_end_sample
pub fn f_010ac130(cpu: &mut Cpu) { sample_status(cpu) } // AIL_sample_status
pub fn f_010abc70(cpu: &mut Cpu) { master_volume(cpu) } // AIL_set_digital_master_volume

// XA_3DFX.EXE E 1.1.0.
pub fn f_020a99d0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_MDI_INI
pub fn f_020abfd0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_DIG_INI
pub fn f_020a9710(cpu: &mut Cpu) { allocate_sequence(cpu) } // AIL_allocate_sequence_handle
pub fn f_020abd10(cpu: &mut Cpu) { allocate_sample(cpu) } // AIL_allocate_sample_handle
pub fn f_020abb40(cpu: &mut Cpu) { release_sample(cpu) } // AIL_release_sample_handle
pub fn f_020a9660(cpu: &mut Cpu) { release_sequence(cpu) } // AIL_release_sequence_handle
pub fn f_020abe10(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_DIG_driver
pub fn f_020a9810(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_MDI_driver
pub fn f_020a8790(cpu: &mut Cpu) { nothing(cpu) } // AIL_set_GTL_filename_prefix
pub fn f_020a9540(cpu: &mut Cpu) { init_sequence(cpu) } // AIL_init_sequence
pub fn f_020a9490(cpu: &mut Cpu) { start_sequence(cpu) } // AIL_start_sequence
pub fn f_020a93e0(cpu: &mut Cpu) { stop_sequence(cpu) } // AIL_stop_sequence
pub fn f_020a9330(cpu: &mut Cpu) { resume_sequence(cpu) } // AIL_resume_sequence
pub fn f_020a9280(cpu: &mut Cpu) { end_sequence(cpu) } // AIL_end_sequence
pub fn f_020a90e0(cpu: &mut Cpu) { set_sequence_volume(cpu) } // AIL_set_sequence_volume
pub fn f_020a9020(cpu: &mut Cpu) { set_sequence_loops(cpu) } // AIL_set_sequence_loop_count
pub fn f_020aba90(cpu: &mut Cpu) { init_sample(cpu) } // AIL_init_sample
pub fn f_020ab970(cpu: &mut Cpu) { set_sample_file(cpu) } // AIL_set_sample_file
pub fn f_020ab8a0(cpu: &mut Cpu) { set_sample_address(cpu) } // AIL_set_sample_address
pub fn f_020ab450(cpu: &mut Cpu) { set_sample_rate(cpu) } // AIL_set_sample_playback_rate
pub fn f_020ab390(cpu: &mut Cpu) { set_sample_level(cpu, false) } // AIL_set_sample_volume
pub fn f_020ab2d0(cpu: &mut Cpu) { set_sample_level(cpu, true) } // AIL_set_sample_pan
pub fn f_020ab210(cpu: &mut Cpu) { set_sample_loops(cpu) } // AIL_set_sample_loop_count
pub fn f_020ab720(cpu: &mut Cpu) { start_sample(cpu) } // AIL_start_sample
pub fn f_020ab510(cpu: &mut Cpu) { end_sample(cpu) } // AIL_end_sample
pub fn f_020ab040(cpu: &mut Cpu) { sample_status(cpu) } // AIL_sample_status
pub fn f_020aab80(cpu: &mut Cpu) { master_volume(cpu) } // AIL_set_digital_master_volume

// The software renderer's programs, `XA.EXE`: the same Miles, found through
// the same debug strings (each wrapper is a fixed distance before the push of
// its string, as in E 1.1.0's XA_3DFX.EXE).

// XA.EXE 1.22 (patched).
pub fn f_03099ca0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_MDI_INI
pub fn f_0309c2a0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_DIG_INI
pub fn f_030999e0(cpu: &mut Cpu) { allocate_sequence(cpu) } // AIL_allocate_sequence_handle
pub fn f_0309bfe0(cpu: &mut Cpu) { allocate_sample(cpu) } // AIL_allocate_sample_handle
pub fn f_0309be10(cpu: &mut Cpu) { release_sample(cpu) } // AIL_release_sample_handle
pub fn f_03099930(cpu: &mut Cpu) { release_sequence(cpu) } // AIL_release_sequence_handle
pub fn f_0309c0e0(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_DIG_driver
pub fn f_03099ae0(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_MDI_driver
pub fn f_03098a60(cpu: &mut Cpu) { nothing(cpu) } // AIL_set_GTL_filename_prefix
pub fn f_03099810(cpu: &mut Cpu) { init_sequence(cpu) } // AIL_init_sequence
pub fn f_03099760(cpu: &mut Cpu) { start_sequence(cpu) } // AIL_start_sequence
pub fn f_030996b0(cpu: &mut Cpu) { stop_sequence(cpu) } // AIL_stop_sequence
pub fn f_03099600(cpu: &mut Cpu) { resume_sequence(cpu) } // AIL_resume_sequence
pub fn f_03099550(cpu: &mut Cpu) { end_sequence(cpu) } // AIL_end_sequence
pub fn f_030993b0(cpu: &mut Cpu) { set_sequence_volume(cpu) } // AIL_set_sequence_volume
pub fn f_030992f0(cpu: &mut Cpu) { set_sequence_loops(cpu) } // AIL_set_sequence_loop_count
pub fn f_0309bd60(cpu: &mut Cpu) { init_sample(cpu) } // AIL_init_sample
pub fn f_0309bc40(cpu: &mut Cpu) { set_sample_file(cpu) } // AIL_set_sample_file
pub fn f_0309bb70(cpu: &mut Cpu) { set_sample_address(cpu) } // AIL_set_sample_address
pub fn f_0309b720(cpu: &mut Cpu) { set_sample_rate(cpu) } // AIL_set_sample_playback_rate
pub fn f_0309b660(cpu: &mut Cpu) { set_sample_level(cpu, false) } // AIL_set_sample_volume
pub fn f_0309b5a0(cpu: &mut Cpu) { set_sample_level(cpu, true) } // AIL_set_sample_pan
pub fn f_0309b4e0(cpu: &mut Cpu) { set_sample_loops(cpu) } // AIL_set_sample_loop_count
pub fn f_0309b9f0(cpu: &mut Cpu) { start_sample(cpu) } // AIL_start_sample
pub fn f_0309b7e0(cpu: &mut Cpu) { end_sample(cpu) } // AIL_end_sample
pub fn f_0309b310(cpu: &mut Cpu) { sample_status(cpu) } // AIL_sample_status
pub fn f_0309ae50(cpu: &mut Cpu) { master_volume(cpu) } // AIL_set_digital_master_volume

// XA.EXE E 1.1.0, English CD.
pub fn f_040994c0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_MDI_INI
pub fn f_0409bac0(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_DIG_INI
pub fn f_04099200(cpu: &mut Cpu) { allocate_sequence(cpu) } // AIL_allocate_sequence_handle
pub fn f_0409b800(cpu: &mut Cpu) { allocate_sample(cpu) } // AIL_allocate_sample_handle
pub fn f_0409b630(cpu: &mut Cpu) { release_sample(cpu) } // AIL_release_sample_handle
pub fn f_04099150(cpu: &mut Cpu) { release_sequence(cpu) } // AIL_release_sequence_handle
pub fn f_0409b900(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_DIG_driver
pub fn f_04099300(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_MDI_driver
pub fn f_04098280(cpu: &mut Cpu) { nothing(cpu) } // AIL_set_GTL_filename_prefix
pub fn f_04099030(cpu: &mut Cpu) { init_sequence(cpu) } // AIL_init_sequence
pub fn f_04098f80(cpu: &mut Cpu) { start_sequence(cpu) } // AIL_start_sequence
pub fn f_04098ed0(cpu: &mut Cpu) { stop_sequence(cpu) } // AIL_stop_sequence
pub fn f_04098e20(cpu: &mut Cpu) { resume_sequence(cpu) } // AIL_resume_sequence
pub fn f_04098d70(cpu: &mut Cpu) { end_sequence(cpu) } // AIL_end_sequence
pub fn f_04098bd0(cpu: &mut Cpu) { set_sequence_volume(cpu) } // AIL_set_sequence_volume
pub fn f_04098b10(cpu: &mut Cpu) { set_sequence_loops(cpu) } // AIL_set_sequence_loop_count
pub fn f_0409b580(cpu: &mut Cpu) { init_sample(cpu) } // AIL_init_sample
pub fn f_0409b460(cpu: &mut Cpu) { set_sample_file(cpu) } // AIL_set_sample_file
pub fn f_0409b390(cpu: &mut Cpu) { set_sample_address(cpu) } // AIL_set_sample_address
pub fn f_0409af40(cpu: &mut Cpu) { set_sample_rate(cpu) } // AIL_set_sample_playback_rate
pub fn f_0409ae80(cpu: &mut Cpu) { set_sample_level(cpu, false) } // AIL_set_sample_volume
pub fn f_0409adc0(cpu: &mut Cpu) { set_sample_level(cpu, true) } // AIL_set_sample_pan
pub fn f_0409ad00(cpu: &mut Cpu) { set_sample_loops(cpu) } // AIL_set_sample_loop_count
pub fn f_0409b210(cpu: &mut Cpu) { start_sample(cpu) } // AIL_start_sample
pub fn f_0409b000(cpu: &mut Cpu) { end_sample(cpu) } // AIL_end_sample
pub fn f_0409ab30(cpu: &mut Cpu) { sample_status(cpu) } // AIL_sample_status
pub fn f_0409a670(cpu: &mut Cpu) { master_volume(cpu) } // AIL_set_digital_master_volume

// XA.EXE R 1.0.1, German CD.
pub fn f_05092770(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_MDI_INI
pub fn f_05094d70(cpu: &mut Cpu) { install_driver(cpu) } // AIL_install_DIG_INI
pub fn f_050924b0(cpu: &mut Cpu) { allocate_sequence(cpu) } // AIL_allocate_sequence_handle
pub fn f_05094ab0(cpu: &mut Cpu) { allocate_sample(cpu) } // AIL_allocate_sample_handle
pub fn f_050948e0(cpu: &mut Cpu) { release_sample(cpu) } // AIL_release_sample_handle
pub fn f_05092400(cpu: &mut Cpu) { release_sequence(cpu) } // AIL_release_sequence_handle
pub fn f_05094bb0(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_DIG_driver
pub fn f_050925b0(cpu: &mut Cpu) { nothing(cpu) } // AIL_uninstall_MDI_driver
pub fn f_05091530(cpu: &mut Cpu) { nothing(cpu) } // AIL_set_GTL_filename_prefix
pub fn f_050922e0(cpu: &mut Cpu) { init_sequence(cpu) } // AIL_init_sequence
pub fn f_05092230(cpu: &mut Cpu) { start_sequence(cpu) } // AIL_start_sequence
pub fn f_05092180(cpu: &mut Cpu) { stop_sequence(cpu) } // AIL_stop_sequence
pub fn f_050920d0(cpu: &mut Cpu) { resume_sequence(cpu) } // AIL_resume_sequence
pub fn f_05092020(cpu: &mut Cpu) { end_sequence(cpu) } // AIL_end_sequence
pub fn f_05091e80(cpu: &mut Cpu) { set_sequence_volume(cpu) } // AIL_set_sequence_volume
pub fn f_05091dc0(cpu: &mut Cpu) { set_sequence_loops(cpu) } // AIL_set_sequence_loop_count
pub fn f_05094830(cpu: &mut Cpu) { init_sample(cpu) } // AIL_init_sample
pub fn f_05094710(cpu: &mut Cpu) { set_sample_file(cpu) } // AIL_set_sample_file
pub fn f_05094640(cpu: &mut Cpu) { set_sample_address(cpu) } // AIL_set_sample_address
pub fn f_050941f0(cpu: &mut Cpu) { set_sample_rate(cpu) } // AIL_set_sample_playback_rate
pub fn f_05094130(cpu: &mut Cpu) { set_sample_level(cpu, false) } // AIL_set_sample_volume
pub fn f_05094070(cpu: &mut Cpu) { set_sample_level(cpu, true) } // AIL_set_sample_pan
pub fn f_05093fb0(cpu: &mut Cpu) { set_sample_loops(cpu) } // AIL_set_sample_loop_count
pub fn f_050944c0(cpu: &mut Cpu) { start_sample(cpu) } // AIL_start_sample
pub fn f_050942b0(cpu: &mut Cpu) { end_sample(cpu) } // AIL_end_sample
pub fn f_05093de0(cpu: &mut Cpu) { sample_status(cpu) } // AIL_sample_status
pub fn f_05093920(cpu: &mut Cpu) { master_volume(cpu) } // AIL_set_digital_master_volume
