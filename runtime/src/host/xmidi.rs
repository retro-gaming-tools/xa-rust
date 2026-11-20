//! XMIDI, the Miles Sound System's music format, read from the game's memory
//! and played by the MIDI synthesizer (see `midi`).
//!
//! A file is an IFF tree: an optional `FORM XDIR`, then a `CAT XMID` of
//! `FORM XMID` songs, each with an `EVNT` chunk. Events are MIDI with three
//! differences: time passes in bytes below 0x80, summed, at a fixed 120 ticks
//! a second; a note-on carries its length, so there are no note-offs; and
//! there's no running status. Controllers 0x6E to 0x78 are Miles' own
//! (channel locks, loops, callbacks) and don't go to the synthesizer.

use crate::cpu::Cpu;

/// Miles plays XMIDI at a fixed 120 ticks a second.
pub const TICKS_PER_SECOND: f64 = 120.0;
const END_OF_TRACK: u8 = 0x2f;
const FIRST_MILES_CONTROLLER: u8 = 0x6e;
const LAST_MILES_CONTROLLER: u8 = 0x78;
/// Further than any song, against a damaged file.
const LONGEST: u32 = 4 << 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    /// A MIDI message: status, and up to two data bytes.
    Message(u8, u8, u8),
    /// A note-on, and how many ticks the note lasts.
    Note(u8, u8, u8, u32),
}

/// A song's events, each with its time in ticks from the start, in order,
/// and when the song ends.
#[derive(Debug, Default, PartialEq)]
pub struct Song {
    pub events: Vec<(u32, Event)>,
    pub length: u32,
}

fn be32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// The `EVNT` chunk of song `number` (from 0), in a file's bytes.
fn find_events(bytes: &[u8], number: u32) -> Option<&[u8]> {
    let mut found = 0;
    let mut events = None;
    walk(bytes, 0, bytes.len(), &mut |kind, body| {
        if kind == b"EVNT" && events.is_none() {
            if found == number {
                events = Some(body);
            }
            found += 1;
        }
    });
    events
}

/// Calls `chunk` with each leaf chunk's kind and body, in order, going into
/// `FORM` and `CAT ` groups.
fn walk<'a>(bytes: &'a [u8], mut at: usize, end: usize, chunk: &mut dyn FnMut(&[u8], &'a [u8])) {
    while at + 8 <= end {
        let kind = &bytes[at..at + 4];
        let Some(size) = be32(bytes, at + 4) else { return };
        let body_end = (at + 8).saturating_add(size as usize).min(end);
        if kind == b"FORM" || kind == b"CAT " {
            walk(bytes, at + 12, body_end, chunk);
        } else {
            chunk(kind, &bytes[at + 8..body_end]);
        }
        at = body_end + (size as usize & 1);
    }
}

/// A standard MIDI variable-length number.
fn number(bytes: &[u8], at: &mut usize) -> Option<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = *bytes.get(*at)?;
        *at += 1;
        value = value << 7 | (byte & 0x7f) as u32;
        if byte < 0x80 {
            return Some(value);
        }
    }
    None
}

/// Reads song `number` of an XMIDI file.
pub fn parse(bytes: &[u8], number_of_song: u32) -> Option<Song> {
    let events = find_events(bytes, number_of_song)?;
    let mut song = Song::default();
    let (mut at, mut time) = (0usize, 0u32);
    while at < events.len() {
        let status = events[at];
        at += 1;
        if status < 0x80 {
            time = time.saturating_add(status as u32);
            continue;
        }
        let data = |at: &mut usize| -> Option<u8> {
            let byte = *events.get(*at)?;
            *at += 1;
            Some(byte & 0x7f)
        };
        match status & 0xf0 {
            0x90 => {
                let (note, velocity) = (data(&mut at)?, data(&mut at)?);
                let length = number(events, &mut at)?;
                song.events.push((time, Event::Note(status, note, velocity, length)));
                song.length = song.length.max(time.saturating_add(length));
            }
            0x80 | 0xa0 | 0xe0 => {
                let (first, second) = (data(&mut at)?, data(&mut at)?);
                song.events.push((time, Event::Message(status, first, second)));
            }
            0xb0 => {
                let (controller, value) = (data(&mut at)?, data(&mut at)?);
                if !(FIRST_MILES_CONTROLLER..=LAST_MILES_CONTROLLER).contains(&controller) {
                    song.events.push((time, Event::Message(status, controller, value)));
                }
            }
            0xc0 | 0xd0 => {
                let first = data(&mut at)?;
                song.events.push((time, Event::Message(status, first, 0)));
            }
            _ => match status {
                0xff => {
                    let kind = *events.get(at)?;
                    at += 1;
                    let length = number(events, &mut at)?;
                    at += length as usize;
                    if kind == END_OF_TRACK {
                        break;
                    }
                }
                0xf0 | 0xf7 => {
                    let length = number(events, &mut at)?;
                    at += length as usize;
                }
                _ => return None,
            },
        }
        if time > LONGEST {
            return None;
        }
    }
    song.length = song.length.max(time);
    Some(song)
}

/// Reads song `number` of an XMIDI file in the game's memory at `at`. The
/// file's size comes from its own chunks.
pub fn parse_at(cpu: &Cpu, at: u32, number_of_song: u32) -> Option<Song> {
    // The top-level chunks: an optional FORM XDIR, then the CAT or FORM.
    let mut size = 0u32;
    for _ in 0..2 {
        let kind = cpu.bytes(at + size, 4);
        if kind != b"FORM" && kind != b"CAT " {
            break;
        }
        let length = u32::from_be_bytes(cpu.bytes(at + size + 4, 4).try_into().ok()?);
        if length > LONGEST {
            return None;
        }
        size += 8 + length + (length & 1);
    }
    parse(cpu.bytes(at, size), number_of_song)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(kind: &[u8], body: &[u8]) -> Vec<u8> {
        let mut bytes = kind.to_vec();
        bytes.extend((body.len() as u32).to_be_bytes());
        bytes.extend(body);
        if body.len() & 1 == 1 {
            bytes.push(0);
        }
        bytes
    }

    fn file(events: &[u8]) -> Vec<u8> {
        let mut xdir = b"XDIR".to_vec();
        xdir.extend(chunk(b"INFO", &[1, 0]));
        let mut form = b"XMID".to_vec();
        form.extend(chunk(b"EVNT", events));
        let mut cat = b"XMID".to_vec();
        cat.extend(chunk(b"FORM", &form));
        let mut bytes = chunk(b"FORM", &xdir);
        bytes.extend(chunk(b"CAT ", &cat));
        bytes
    }

    #[test]
    fn times_notes_and_controllers() {
        // A program, 60+60 ticks, a note of 0x81 0x00 = 128 ticks, a Miles
        // controller that's dropped, a volume, the end.
        let events = [0xc0, 48, 60, 60, 0x90, 64, 100, 0x81, 0x00, 0xb0, 0x74, 0, 0xb0, 7, 90, 0xff, 0x2f, 0];
        let song = parse(&file(&events), 0).unwrap();
        assert_eq!(song.events, vec![(0, Event::Message(0xc0, 48, 0)), (120, Event::Note(0x90, 64, 100, 128)), (120, Event::Message(0xb0, 7, 90))]);
        assert_eq!(song.length, 248);
    }

    #[test]
    fn damaged_files_give_nothing() {
        assert_eq!(parse(&file(&[0x90, 64]), 0), None);
        assert_eq!(parse(&file(&[0xc0, 1, 0xff, 0x2f, 0]), 1), None);
        assert_eq!(parse(b"FORM", 0), None);
        assert_eq!(parse(&file(&[0xf4]), 0), None);
    }
}
