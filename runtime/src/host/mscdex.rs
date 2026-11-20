//! MSCDEX, DOS's CD-ROM driver: `int 2Fh` with AH=15h.
//!
//! DOS games play their music as CD audio through it: they read the disc's
//! table of contents, and send the drive "play" and "stop" requests. There's
//! one drive, D:, and its audio tracks are the runtime's music files (see
//! `audio`), or the disc image's own (`cue`). The table of contents is the
//! image's when there is one, or else the real disc's when the port gives it
//! (`recompCdTracks`), because some games check their disc by its track
//! lengths: Extreme Assault wants tracks 2 to 4 within a second of its own.
//! Otherwise it's made up, with every track the same length, which is enough
//! for a game that only works out where a track starts and how much to play,
//! and gets back the same numbers it was given.

use super::vga::Registers;
use crate::cpu::Cpu;

/// Where each track starts and the disc ends, in sectors: the image's, or
/// the port's, or empty for the made-up one.
fn layout() -> &'static [u32] {
    super::cue::disc().map_or_else(crate::loader::cd_tracks, |sheet| sheet.starts.as_slice())
}

/// The CD drive's number: 3 is D:.
const DRIVE: u32 = 3;
/// Sectors a second on an audio disc, and before its first track.
const PER_SECOND: u32 = 75;
const LEAD_IN: u32 = 150;
/// Each track's made-up length: ten minutes.
const TRACK_SECTORS: u32 = 10 * 60 * PER_SECOND;
const DONE: u16 = 0x0100;
const BUSY: u16 = 0x0200;
const ERROR_UNKNOWN_COMMAND: u16 = 0x8003;

/// Where a track starts, in sectors from the start of the disc. Track 1 is
/// the data track, and the track after the last is where the disc ends.
fn track_start(track: u32) -> u32 {
    let layout = layout();
    match layout.get(track.max(1) as usize - 1) {
        Some(&start) => start,
        None if !layout.is_empty() => layout[layout.len() - 1],
        None => (track - 1) * TRACK_SECTORS,
    }
}

/// The track a sector is in.
fn track_at(sector: u32) -> u32 {
    let layout = layout();
    if layout.is_empty() {
        return sector / TRACK_SECTORS + 1;
    }
    let tracks = layout.len() as u32 - 1;
    (1..=tracks).rev().find(|&track| track_start(track) <= sector).unwrap_or(1)
}

/// The track a play starts in. A start up to a pregap (two seconds) before
/// a track counts as that track: Miles works in milliseconds, and asks
/// Extreme Assault's drive to play from 20 sectors before track 20.
fn first_track_at(sector: u32) -> u32 {
    const SLACK: u32 = 2 * PER_SECOND;
    track_at(sector + if layout().is_empty() { 0 } else { SLACK })
}

/// A place on the disc as minute, second and frame, packed as the driver
/// does: frame in the lowest byte.
fn red_book(sector: u32) -> u32 {
    let sector = sector + LEAD_IN;
    (sector / (60 * PER_SECOND)) << 16 | (sector / PER_SECOND % 60) << 8 | sector % PER_SECOND
}

fn from_red_book(value: u32) -> u32 {
    ((value >> 16 & 0xff) * 60 * PER_SECOND + (value >> 8 & 0xff) * PER_SECOND + (value & 0xff)).saturating_sub(LEAD_IN)
}

/// The tracks a disc has: the real disc's, or else the data track and the
/// music files.
fn last_track(cpu: &Cpu) -> u32 {
    match layout().len() {
        0 => super::audio::cd_tracks(cpu).max(1),
        places => places as u32 - 1,
    }
}

/// Reading from the drive: the control block's first byte says what.
fn read_control(cpu: &mut Cpu, block: u32) {
    match cpu.r8(block) {
        // Where the head is, as a sector.
        1 => {
            cpu.w8(block + 1, 0);
            let at = super::audio::cd_playing(cpu).map_or(0, track_start);
            cpu.w32(block + 2, at);
        }
        // The drive's abilities: it plays audio and takes both address forms.
        // With no layout and no music, it says there's no disc in it.
        6 => {
            let empty = layout().is_empty() && super::audio::cd_tracks(cpu) == 0;
            cpu.w32(block + 1, 0x0000_0210 | if empty { 0x800 } else { 0 });
        }
        // Sector size, and the disc's size.
        7 => cpu.w16(block + 2, 2048),
        8 => {
            let end = track_start(last_track(cpu) + 1);
            cpu.w32(block + 1, end);
        }
        // Has the disc been changed? No.
        9 => cpu.w8(block + 1, 1),
        // The disc: its first and last track, and where it ends.
        0x0a => {
            let last = last_track(cpu);
            cpu.w8(block + 1, 1);
            cpu.w8(block + 2, last as u8);
            cpu.w32(block + 3, red_book(track_start(last + 1)));
        }
        // One track: where it starts, and whether it's data.
        0x0b => {
            let track = cpu.r8(block + 1) as u32;
            cpu.w32(block + 2, red_book(track_start(track.max(1))));
            cpu.w8(block + 6, if track <= 1 { 0x40 } else { 0 });
        }
        // What's playing now: the track, in the driver's packed decimal.
        0x0c => {
            let track = super::audio::cd_playing(cpu).unwrap_or(1);
            cpu.w8(block + 1, 0);
            cpu.w8(block + 2, (track / 10 << 4 | track % 10) as u8);
            cpu.w8(block + 3, 1);
            for offset in 4..11 {
                cpu.w8(block + offset, 0);
            }
        }
        // Whether it's paused, and what it would resume.
        0x0f => {
            cpu.w16(block + 1, 0);
            cpu.w32(block + 3, 0);
            cpu.w32(block + 7, 0);
        }
        // Each channel's volume: full.
        4 => {
            for channel in 0..4 {
                cpu.w8(block + 1 + channel * 2, channel as u8);
                cpu.w8(block + 2 + channel * 2, 0xff);
            }
        }
        other => println!("  MSCDEX read of control block {other:#x} isn't answered"),
    }
}

/// A request to the drive, in DOS's device driver form.
fn request(cpu: &mut Cpu, header: u32) {
    const COMMAND: u32 = 2;
    const STATUS: u32 = 3;
    const MODE: u32 = 13;
    const TRANSFER: u32 = 14;
    let transfer = cpu.r16(header + TRANSFER) as u32 + ((cpu.r16(header + TRANSFER + 2) as u32) << 4);
    let mut status = DONE;
    match cpu.r8(header + COMMAND) {
        3 => read_control(cpu, transfer),
        // Writing to the drive: eject, reset, and the channels' volume, which
        // sets the music's.
        12 => {
            if cpu.r8(transfer) == 3 {
                let volume = cpu.r8(transfer + 2) as u32;
                super::audio::cd_volume(cpu, volume * 0x101);
            }
        }
        // Play: from a sector, for a number of sectors.
        132 => {
            let start = cpu.r32(header + TRANSFER);
            let start = if cpu.r8(header + MODE) == 1 { from_red_book(start) } else { start };
            let sectors = cpu.r32(header + 18);
            let first = first_track_at(start);
            let end = (track_at(start + sectors.max(1) - 1) + 1).max(first + 1);
            if cpu.trace {
                println!("  CD play from sector {start} for {sectors}: tracks {first} to {}", end - 1);
            }
            super::audio::cd_play(cpu, first, end);
        }
        133 => super::audio::cd_stop(cpu),
        // Resume, seek, and flushing buffers: nothing to do.
        136 | 131 | 7 | 11 | 13 | 14 => {}
        other => {
            println!("  MSCDEX request {other} isn't answered");
            status = ERROR_UNKNOWN_COMMAND;
        }
    }
    if super::audio::cd_playing(cpu).is_some() {
        status |= BUSY;
    }
    cpu.w16(header + STATUS, status);
}

/// `int 2Fh` AH=15h, with the buffer the program gave at ES:BX.
pub fn call(cpu: &mut Cpu, r: &mut Registers, buffer: u32) -> bool {
    match r.eax & 0xff {
        // Is it there? One drive, D:.
        0x00 => {
            r.ebx = (r.ebx & !0xffff) | 1;
            r.ecx = (r.ecx & !0xffff) | DRIVE;
        }
        // Is this drive a CD?
        0x0b => {
            r.ebx = (r.ebx & !0xffff) | 0xadad;
            r.eax = (r.eax & !0xffff) | if r.ecx & 0xffff == DRIVE { 0xffff } else { 0 };
        }
        // Its version, 2.23.
        0x0c => r.ebx = (r.ebx & !0xffff) | 0x0217,
        // The CD drives' letters.
        0x0d => cpu.w8(buffer, DRIVE as u8),
        0x10 => request(cpu, buffer),
        _ => return false,
    }
    true
}
