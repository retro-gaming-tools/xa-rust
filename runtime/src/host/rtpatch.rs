//! Extreme Assault's German 3dfx patch, taken from the file players download
//! (`xa_r110.zip`, the `XA_R110.EXE` in it, or that program's `PATCH.EXE`),
//! so the German disc gets its Glide program without running the 16-bit
//! patcher, which Windows can't.
//!
//! `XA_R110.EXE` is a self-extracting zip, and its `PATCH.EXE` is Pocket
//! Soft's .RTPatch 3.20. Each file in it is a compressed stream: a whole file
//! (`XA_3DFX.EXE`, `GLIDE2X.OVL`, the 3dfx cockpit pictures), or a program of
//! copy and change steps against the German disc's own file (`GFX.CHR`,
//! `INGAME.CHR`, `VORHLT*.DAT`). The patch is one known file, so where each
//! stream starts is in `FILES`, and every result is checked by its SHA-256
//! before it's written. Nothing of the game is in this file.
//!
//! The stream codec and the step program are as worked out by rtptool
//! (https://github.com/bwrsandman/rtptool), whose `codec.rs` and `apply.rs`
//! this follows, under its licence:
//!
//! MIT License
//!
//! Copyright (c) 2026 Sandy Carter
//!
//! Permission is hereby granted, free of charge, to any person obtaining a
//! copy of this software and associated documentation files (the
//! "Software"), to deal in the Software without restriction, including
//! without limitation the rights to use, copy, modify, merge, publish,
//! distribute, sublicense, and/or sell copies of the Software, and to permit
//! persons to whom the Software is furnished to do so, subject to the
//! following conditions:
//!
//! The above copyright notice and this permission notice shall be included
//! in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//! OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
//! MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
//! NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//! DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
//! OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
//! USE OR OTHER DEALINGS IN THE SOFTWARE.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Where players get the patch. The port never downloads it.
pub const DOWNLOAD: &str = "patches-scrolls.de/patch/1516/7/24201";

/// `PATCH.EXE` from `XA_R110.EXE`.
const PATCH_SHA256: &str = "99e140d18ca81f2c5fcdd2d20b4ebc7ddc2f3fc8324746635cab9c62fa101397";
const PATCH_SIZE: usize = 1_207_719;

/// The German disc's program, which tells a folder that wants the patch.
const GERMAN_XA_SHA256: &str = "e69a049251c8ade591adb5e5ee02fed8b05d3313ba86ab4e6cd02408fbefd725";
/// The patch's Glide program.
const GLIDE_PROGRAM: &str = "XA_3DFX.EXE";

struct File {
    /// Where its stream starts in `PATCH.EXE`.
    at: usize,
    /// Its place in the installed game.
    path: &'static str,
    size: usize,
    /// For a changed file, the German disc's file it's made from.
    old_sha256: Option<&'static str>,
    sha256: &'static str,
}

const fn whole(at: usize, path: &'static str, size: usize, sha256: &'static str) -> File {
    File { at, path, size, old_sha256: None, sha256 }
}

const fn changed(at: usize, path: &'static str, size: usize, old: &'static str, sha256: &'static str) -> File {
    File { at, path, size, old_sha256: Some(old), sha256 }
}

/// What the patch gives the German disc for Glide. Left out: its `XA.EXE`
/// (1.10's software program, which the port doesn't recompile), joystick
/// drivers' files, `.BAT` files and its notes.
const FILES: &[File] = &[
    whole(0x099896, "XA_3DFX.EXE", 1_344_528, "24f23a78015595cd8b53c04e3a3bf96a48d055f18fc0c29faecc6df1c947f000"),
    whole(0x01b0fe, "GLIDE2X.OVL", 344_351, "42792d4f8a5f8747e8a3926329e2c38018bae7010d3b4e3f1e7258b9bd4a6a92"),
    whole(0x00f2b3, "DATA.SFX/SOUND/BEEP3.WAV", 336, "8beba7081e0cc5bbbf6672c8638e0ed977960328e6d4371fcd980a82167cff00"),
    whole(0x00f4a9, "DATA/GLOBAL/COCKPIT/3DFX/D6-MAIN.RLE", 122_906, "2a73448c919502d357e93609b0c79cf60f519e5117aa05e460f2c9b9ffc223cb"),
    whole(0x018f0d, "DATA/GLOBAL/COCKPIT/3DFX/D6-SHW.RLE", 23_810, "eec544e0dfa65180300ce36400cc256bac39fd127cc911a6f6fda92a6187b693"),
    whole(0x04253a, "DATA/GLOBAL/COCKPIT/3DFX/HI.RLE", 109_924, "3cad21a2d1eb2dc8334707edecc82bd8a2f2cb7c0c16088515100ca882760253"),
    whole(0x0480e2, "DATA/GLOBAL/COCKPIT/3DFX/HI_NET.RLE", 100_320, "17c37055d9c52b25fc019318d10aff359fde6207720beaecd75c503d9f23258a"),
    whole(0x04d568, "DATA/GLOBAL/COCKPIT/3DFX/HI_SHD.RLE", 16_752, "e6a9324cb0feb9f0821cdbdbe1692e72314bae580a74399b27cf2bb187127db9"),
    changed(0x01adfc, "DATA/MENU/TEXT/GFX.CHR", 11_789,
        "d9a3dd9cffccc67d84cf4ec63d919b956034680889ba9ac8de4442da314dde01", "53d7b46c29181893412a4cc25fe4cc220e6edf007aee83cec126c9e70ff9d7a7"),
    changed(0x04e930, "DATA/GLOBAL/TEXT/INGAME.CHR", 2_683,
        "546deb5d5bec87226edb773baeb1546e792079128f4ae597eeed6c2ef84889a1", "ce9ac102278343f9cea9b75039a84c14ebb7dc89de432fcb99d16ae9ae751be6"),
    changed(0x0500b8, "DATA/GLOBAL/COCKPIT/ITEM/HIRES/VORHLT.DAT", 1_586,
        "9a58dd571f46c34ceb8e92663c0d628d917a78868c3db1c7f7097ac10475e223", "8ac3b59aad478b2ae53976f76a6da6099cc61f69745104f985526ff11eaade18"),
    changed(0x05022b, "DATA/GLOBAL/COCKPIT/ITEM/LORES/VORHLTLO.DAT", 914,
        "6dc7d62649a2dcf205c29c8314853ca5276a3bc5516e44e6998353905f701420", "a827da879d8d43465d18d5e45d434b3f7d91fe9a0c8f4e966b40c16966a87ef8"),
];

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

/// `path` under `root`, each part matched without regard to case, if it's there.
fn find(root: &Path, path: &str) -> Option<PathBuf> {
    let mut at = root.to_path_buf();
    for part in path.split('/') {
        let exact = at.join(part);
        at = if exact.exists() {
            exact
        } else {
            std::fs::read_dir(&at).ok()?.flatten().map(|entry| entry.path())
                .find(|candidate| candidate.file_name().is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case(part)))?
        };
    }
    Some(at)
}

/// A file of the game: in the folder as installed, or on an unpacked disc,
/// whose installer copied `EXTREME/ROOT` to the root and read the rest from
/// `EXTREME`.
fn game_file(folder: &Path, path: &str) -> Option<PathBuf> {
    find(folder, path)
        .or_else(|| find(folder, &format!("EXTREME/ROOT/{path}")))
        .or_else(|| find(folder, &format!("EXTREME/{path}")))
}

/// Whether the folder holds the German disc's game without the Glide program.
pub fn wanted(folder: &str) -> bool {
    let folder = Path::new(folder);
    if !folder.is_dir() || game_file(folder, GLIDE_PROGRAM).is_some() {
        return false;
    }
    game_file(folder, "XA.EXE").and_then(|program| std::fs::read(program).ok()).is_some_and(|bytes| sha256(&bytes) == GERMAN_XA_SHA256)
}

/// `PATCH.EXE` from what the player picked, if it's the patch: `PATCH.EXE`
/// itself, `XA_R110.EXE` or `xa_r110.zip` (a zip holding `XA_R110.EXE`).
pub fn patch_program(picked: &Path) -> Option<Vec<u8>> {
    let is_patch = |bytes: &[u8]| bytes.len() == PATCH_SIZE && sha256(bytes) == PATCH_SHA256;
    let bytes = std::fs::read(picked).ok()?;
    if is_patch(&bytes) {
        return Some(bytes);
    }
    // XA_R110.EXE holds PATCH.EXE, and xa_r110.zip holds XA_R110.EXE.
    let program = zip_member(&bytes, "XA_R110.EXE").unwrap_or(bytes);
    zip_member(&program, "PATCH.EXE").filter(|patch| is_patch(patch))
}

/// Writes the patch's files into the game folder. Returns how many were
/// written; those already there are left as they are.
pub fn apply_to(folder: &str, patch: &[u8]) -> Result<usize, String> {
    let root = Path::new(folder);
    let mut written = 0;
    for file in FILES {
        let target = find(root, file.path).unwrap_or_else(|| root.join(file.path));
        if std::fs::read(&target).is_ok_and(|bytes| sha256(&bytes) == file.sha256) {
            continue;
        }
        let bytes = match file.old_sha256 {
            None => decompress(patch, file.at, file.size)?,
            Some(old_sha256) => {
                let old = game_file(root, file.path).and_then(|path| std::fs::read(path).ok())
                    .ok_or_else(|| format!("{} isn't in the game folder", file.path))?;
                if sha256(&old) != old_sha256 {
                    return Err(format!("{} isn't the German disc's", file.path));
                }
                let steps = decompress(patch, file.at, 1 << 22)?;
                apply(&old, &steps, file.size)?
            }
        };
        if sha256(&bytes) != file.sha256 {
            return Err(format!("{} came out wrong from the patch", file.path));
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        // The German software program (1.0.1) stops on the patched text files
        // ("TextNr to High"), so an installed original is kept beside them.
        if file.old_sha256.is_some() && target.exists() {
            let kept = keep_path(&target);
            if !kept.exists() {
                std::fs::copy(&target, &kept).map_err(|error| format!("{}: {error}", kept.display()))?;
            }
        }
        std::fs::write(&target, &bytes).map_err(|error| format!("{}: {error}", target.display()))?;
        written += 1;
    }
    Ok(written)
}

/// Where an installed file the patch changes is kept: `GFX.CHR` → `GFX.CHR.101`.
fn keep_path(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".101");
    path.with_file_name(name)
}

/// For the German disc's software program, which only knows 1.0.1's files:
/// the original of a file the patch changed, when `path` holds the patched one.
/// It's the copy `apply_to` kept, or the disc's own in `EXTREME`.
pub fn original_for_software(folder: &Path, path: &Path) -> Option<PathBuf> {
    let relative = path.strip_prefix(folder).ok()?.to_string_lossy().replace('\\', "/");
    let file = FILES.iter().find(|file| file.old_sha256.is_some() && file.path.eq_ignore_ascii_case(&relative))?;
    if !std::fs::read(path).is_ok_and(|bytes| sha256(&bytes) == file.sha256) {
        return None;
    }
    let kept = keep_path(path);
    let disc = find(folder, &format!("EXTREME/{}", file.path));
    [Some(kept), disc].into_iter().flatten()
        .find(|candidate| std::fs::read(candidate).is_ok_and(|bytes| Some(sha256(&bytes).as_str()) == file.old_sha256))
}

// Zip, for the self-extracting XA_R110.EXE: its local headers are found by
// their signature, which also works with the program in front of them.

fn le16(bytes: &[u8], at: usize) -> usize {
    bytes[at] as usize | (bytes[at + 1] as usize) << 8
}

fn le32(bytes: &[u8], at: usize) -> usize {
    le16(bytes, at) | le16(bytes, at + 2) << 16
}

fn zip_member(zip: &[u8], wanted: &str) -> Option<Vec<u8>> {
    let mut at = 0;
    while at + 30 <= zip.len() {
        if zip[at..at + 4] != [0x50, 0x4b, 0x03, 0x04] {
            at += 1;
            continue;
        }
        let flags = le16(zip, at + 6);
        let method = le16(zip, at + 8);
        let packed = le32(zip, at + 18);
        let size = le32(zip, at + 22);
        let name_length = le16(zip, at + 26);
        let extra = le16(zip, at + 28);
        let data = at + 30 + name_length + extra;
        let name = zip.get(at + 30..at + 30 + name_length)?;
        // A size given only after the data (flag 8) isn't used by this zip.
        if flags & 8 == 0 && data + packed <= zip.len() && String::from_utf8_lossy(name).eq_ignore_ascii_case(wanted) {
            let stored = &zip[data..data + packed];
            return match method {
                0 => Some(stored.to_vec()),
                8 => inflate(stored, size).ok(),
                _ => None,
            };
        }
        at += 4;
    }
    None
}

/// Deflate (RFC 1951), enough for a zip's members.
fn inflate(input: &[u8], size: usize) -> Result<Vec<u8>, String> {
    struct Bits<'a> {
        input: &'a [u8],
        at: usize,
        buffer: u32,
        count: u32,
    }
    impl Bits<'_> {
        fn take(&mut self, n: u32) -> Result<u32, String> {
            while self.count < n {
                let byte = *self.input.get(self.at).ok_or("deflate data ends early")?;
                self.at += 1;
                self.buffer |= (byte as u32) << self.count;
                self.count += 8;
            }
            let value = self.buffer & ((1u32 << n) - 1);
            self.buffer >>= n;
            self.count -= n;
            Ok(value)
        }
    }
    /// Code counts per length and the symbols in code order.
    struct Huffman {
        counts: [u16; 16],
        symbols: Vec<u16>,
    }
    impl Huffman {
        fn new(lengths: &[u8]) -> Huffman {
            let mut counts = [0u16; 16];
            for &length in lengths {
                counts[length as usize] += 1;
            }
            counts[0] = 0;
            let mut offsets = [0u16; 16];
            for length in 1..15 {
                offsets[length + 1] = offsets[length] + counts[length];
            }
            let mut symbols = vec![0u16; lengths.len()];
            for (symbol, &length) in lengths.iter().enumerate() {
                if length != 0 {
                    symbols[offsets[length as usize] as usize] = symbol as u16;
                    offsets[length as usize] += 1;
                }
            }
            Huffman { counts, symbols }
        }
        fn decode(&self, bits: &mut Bits) -> Result<u16, String> {
            let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
            for length in 1..16 {
                code |= bits.take(1)? as i32;
                let count = self.counts[length] as i32;
                if code - count < first {
                    return Ok(self.symbols[(index + (code - first)) as usize]);
                }
                index += count;
                first = (first + count) << 1;
                code <<= 1;
            }
            Err("bad deflate code".into())
        }
    }
    const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
    const LENGTH_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
    const DISTANCE_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
    const DISTANCE_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

    let mut bits = Bits { input, at: 0, buffer: 0, count: 0 };
    let mut out = Vec::with_capacity(size);
    loop {
        let last = bits.take(1)?;
        match bits.take(2)? {
            0 => {
                bits.buffer = 0;
                bits.count = 0;
                let length = le16(input, bits.at);
                let start = bits.at + 4;
                out.extend_from_slice(input.get(start..start + length).ok_or("stored block ends early")?);
                bits.at = start + length;
            }
            kind @ (1 | 2) => {
                let (literals, distances) = if kind == 1 {
                    let mut lengths = [0u8; 288];
                    lengths[..144].fill(8);
                    lengths[144..256].fill(9);
                    lengths[256..280].fill(7);
                    lengths[280..].fill(8);
                    (Huffman::new(&lengths), Huffman::new(&[5u8; 30]))
                } else {
                    let literal_count = bits.take(5)? as usize + 257;
                    let distance_count = bits.take(5)? as usize + 1;
                    let code_count = bits.take(4)? as usize + 4;
                    let mut code_lengths = [0u8; 19];
                    for &place in &ORDER[..code_count] {
                        code_lengths[place] = bits.take(3)? as u8;
                    }
                    let codes = Huffman::new(&code_lengths);
                    let mut lengths = Vec::with_capacity(literal_count + distance_count);
                    while lengths.len() < literal_count + distance_count {
                        let symbol = codes.decode(&mut bits)?;
                        let (value, repeat) = match symbol {
                            0..=15 => (symbol as u8, 1),
                            16 => (*lengths.last().ok_or("repeat with nothing before")?, 3 + bits.take(2)? as usize),
                            17 => (0, 3 + bits.take(3)? as usize),
                            _ => (0, 11 + bits.take(7)? as usize),
                        };
                        lengths.extend(std::iter::repeat(value).take(repeat));
                    }
                    (Huffman::new(&lengths[..literal_count]), Huffman::new(&lengths[literal_count..literal_count + distance_count]))
                };
                loop {
                    let symbol = literals.decode(&mut bits)? as usize;
                    if symbol < 256 {
                        out.push(symbol as u8);
                    } else if symbol == 256 {
                        break;
                    } else {
                        let index = symbol - 257;
                        let length = *LENGTH_BASE.get(index).ok_or("bad length")? as usize + bits.take(LENGTH_EXTRA[index] as u32)? as usize;
                        let index = distances.decode(&mut bits)? as usize;
                        let distance = *DISTANCE_BASE.get(index).ok_or("bad distance")? as usize + bits.take(DISTANCE_EXTRA[index] as u32)? as usize;
                        if distance > out.len() {
                            return Err("deflate distance too far".into());
                        }
                        for _ in 0..length {
                            out.push(out[out.len() - distance]);
                        }
                    }
                }
            }
            _ => return Err("bad deflate block".into()),
        }
        if last == 1 {
            return Ok(out);
        }
    }
}

// The .RTPatch stream: an MSB-first bit stream, LZSS with adaptive Huffman
// codes for literals, lengths and distances (rtptool's `codec.rs`).

const STREAM_MAGIC: u32 = 0xb59c;

struct BitIn {
    pos: usize,
    end: usize,
    bl: u32,
}

impl BitIn {
    fn bit(&mut self, buf: &[u8]) -> Result<u8, String> {
        if self.pos >= self.end {
            return Err("patch stream ends early".into());
        }
        let value = (buf[self.pos] >> (self.bl - 1)) & 1;
        self.bl -= 1;
        if self.bl == 0 {
            self.pos += 1;
            self.bl = 8;
        }
        Ok(value)
    }

    fn bits(&mut self, n: u32, buf: &[u8]) -> Result<u32, String> {
        let mut value = 0;
        for _ in 0..n {
            value = (value << 1) | self.bit(buf)? as u32;
        }
        Ok(value)
    }
}

/// The adaptive Huffman tree, laid out in bytes as the patcher's own was.
struct HuffTree {
    m: Vec<u8>,
}

impl HuffTree {
    fn r16u(&self, off: usize) -> u32 {
        self.m[off] as u32 | ((self.m[off + 1] as u32) << 8)
    }

    fn r16s(&self, off: usize) -> i32 {
        let v = self.r16u(off);
        if v & 0x8000 != 0 { v as i32 - 0x10000 } else { v as i32 }
    }

    fn w16(&mut self, off: usize, val: i32) {
        let v = (val & 0xffff) as u16;
        self.m[off] = v as u8;
        self.m[off + 1] = (v >> 8) as u8;
    }

    fn r32(&self, off: usize) -> usize {
        u32::from_le_bytes(self.m[off..off + 4].try_into().unwrap()) as usize
    }

    fn w32(&mut self, off: usize, val: usize) {
        self.m[off..off + 4].copy_from_slice(&(val as u32).to_le_bytes());
    }

    fn new(esc_bits: u32, num_levels: usize, init_period: u32, upd_period: u32) -> Self {
        let alpha = 1usize << esc_bits;
        let off_groupcnt = 0x34usize;
        let off_symtab = num_levels * 2 + 0x34;
        let off_slot = num_levels * 6 + 0x38;
        let off_weight = num_levels * 6 + 0x40 + alpha * 4;
        let off_limit = off_weight + 4 + alpha * 2;
        let mut t = HuffTree { m: vec![0u8; off_limit + 0xc0 + 0x100] };
        t.w16(0x32, init_period as i32);
        t.w16(0x02, init_period as i32);
        t.w16(0x00, init_period as i32);
        t.w16(0x30, upd_period as i32);
        t.w16(0x2e, upd_period as i32);
        t.w16(0x2c, upd_period as i32);
        t.w16(0x06, esc_bits as i32);
        t.w16(0x04, num_levels as i32);
        t.w32(0x0c, off_groupcnt);
        t.w32(0x20, off_symtab);
        t.w32(0x1c, off_slot);
        t.w32(0x18, off_weight);
        t.w32(0x14, off_limit);
        t.w32(0x10, off_limit);
        t.w16(0x0a, 1);
        t.w16(0x08, 1);
        t.w16(off_groupcnt, 2);
        for i in 1..num_levels {
            t.w16(off_groupcnt + i * 2, 0);
        }
        t.w32(off_symtab, off_slot);
        for i in 1..=num_levels {
            t.w32(off_symtab + i * 4, off_slot + 8);
        }
        let wbase = off_weight + alpha * 2;
        t.w32(off_slot, wbase);
        for i in 1..=(alpha + 1) {
            t.w32(off_slot + i * 4, wbase + 2);
        }
        for j in 0..alpha {
            t.w16(off_weight + j * 2, 0x8000);
        }
        t.w16(off_weight + alpha * 2, 0);
        t.w16(off_weight + alpha * 2 + 2, 0);
        t.w16(0x24, alpha as i32);
        let lim = t.r32(0x10);
        for k in 0..0x30 {
            t.w32(lim + k * 4, 0);
        }
        t.build_limits(0);
        t
    }

    fn build_limits(&mut self, start: usize) {
        let gc = self.r32(0x0c);
        let lim = self.r32(0x10);
        let num = self.r16u(0x04) as usize;
        let mut s3: i32 = if start == 0 { 2 } else { self.r16s(lim + (start - 1) * 8) * 2 };
        for level in start..num {
            let s4 = s3 - self.r16s(gc + level * 2);
            self.w16(lim + level * 8, s4);
            s3 = s4 * 2;
            self.w16(lim + level * 8 + 2, s3);
            self.w16(lim + level * 8 + 4, s4 * 4);
            self.w16(lim + level * 8 + 6, s4 * 16);
        }
    }

    /// True when the countdown reaches zero and the tree is to be rebuilt.
    fn update_freq(&mut self, sym: usize) -> bool {
        let w = self.r32(0x18);
        let cur = self.r16u(w + sym * 2);
        self.w16(w + sym * 2, (cur + 1) as i32);
        let cnt = self.r16s(0x00) - 1;
        self.w16(0x00, cnt);
        (cnt & 0xffff) == 0
    }

    /// Registers a new symbol; returns the lowest level for `build_limits`.
    fn add_symbol(&mut self, newsym: usize) -> usize {
        let w = self.r32(0x18);
        let slot = self.r32(0x1c);
        let gc = self.r32(0x0c);
        let st = self.r32(0x20);
        let iv = newsym * 2;
        self.w16(w + iv, 1);
        let n_slots = self.r16u(0x08) as usize;
        self.w32(slot + n_slots * 4, w + iv);
        let n_slots = n_slots + 1;
        self.w16(0x08, n_slots as i32);
        if n_slots == 2 {
            return 0;
        }
        let n_groups = self.r16u(0x0a) as usize;
        let num = self.r16u(0x04) as usize;
        let level = if n_groups < num {
            self.w16(0x0a, (n_groups + 1) as i32);
            n_groups.wrapping_sub(1) & 0xffff
        } else {
            let mut u = n_groups.wrapping_sub(2) & 0xffff;
            while self.r16s(gc + u * 2) == 0 {
                u = u.wrapping_sub(1) & 0xffff;
            }
            u
        };
        let v = self.r16s(gc + level * 2);
        self.w16(gc + level * 2, v - 1);
        let v = self.r16s(gc + 2 + level * 2);
        self.w16(gc + 2 + level * 2, v + 2);
        let v = self.r32(st + (level + 1) * 4);
        self.w32(st + (level + 1) * 4, v.wrapping_sub(4));
        for u in (level + 2)..=num {
            let v = self.r32(st + u * 4);
            self.w32(st + u * 4, v.wrapping_add(4));
        }
        level
    }

    fn rebuild(&mut self) {
        let slot = self.r32(0x1c);
        let symt = self.r32(0x20);
        let gc = self.r32(0x0c);
        let n_slots = self.r16u(0x08) as usize;
        let upd = self.r16s(0x2c);
        let num = self.r16u(0x04) as usize;
        let mut maxw: u32 = 0;
        self.w16(0x2c, upd - 1);

        // Halve the weights when the period says so, and find the largest.
        for i in 0..n_slots {
            let p = self.r32(slot + i * 4);
            let mut wv = self.r16u(p);
            if ((upd - 1) & 0xffff) == 0 {
                wv >>= 1;
                self.w16(p, wv as i32);
            }
            maxw = maxw.max(wv);
        }

        // Partition the slots by weight, heaviest first, a bit at a time.
        if maxw != 0 {
            let mut mask = 0x8000u32;
            while (maxw & mask) == 0 {
                mask = (mask >> 1) | 0x8000;
            }
            let mut la: usize = 0;
            while la < n_slots {
                let pcur = self.r32(slot + la * 4);
                if (self.r16u(pcur) & mask) == 0 {
                    let mut u17 = la + 1;
                    let mut ins = la;
                    if n_slots <= u17 {
                        break;
                    }
                    let mut u5 = ins;
                    while u17 < n_slots {
                        let pp = slot + u17 * 4;
                        let p9 = self.r32(pp);
                        u5 = ins;
                        if (self.r16u(p9) & mask) != 0 {
                            u5 = ins + 1;
                            let pcur_val = self.r32(slot + ins * 4);
                            self.w32(slot + ins * 4, p9);
                            self.w32(pp, pcur_val);
                        }
                        u17 += 1;
                        ins = u5;
                    }
                    if u5 != la {
                        la = u5 - 1;
                    }
                    let nm = mask >> 1;
                    mask = nm | 0x8000;
                    if (nm & 1) != 0 {
                        break;
                    }
                } else {
                    la += 1;
                }
            }
        }

        // Rebalance the groups of each code length.
        let mut la: usize = 0;
        let mut n_groups = self.r16u(0x0a) as usize;
        let mut ustk = n_groups.wrapping_sub(1) & 0xffff;
        let mut moved: i32 = 0;
        if n_groups != 0 {
            loop {
                let pgc = gc + la * 2;
                let p2 = symt + la * 4;
                let g = self.r16u(pgc) as usize;
                if g == 0 {
                    la += 1;
                } else {
                    let p1 = self.r32(p2 + 4);
                    let w_first = self.r16u(self.r32(self.r32(p2)));
                    let w_last = self.r16u(self.r32(p1.wrapping_sub(4)));
                    let w_last2 = self.r16u(self.r32(p1.wrapping_sub(8)));
                    if g < 3 || ((num.wrapping_sub(1)) & 0xffff) == la || w_first < w_last + w_last2 {
                        let mut p16 = p2 + 4;
                        let mut found = false;
                        let mut acc = self.r16u(self.r32(p1.wrapping_sub(4))) as i32;
                        let mut u17 = la + 2;
                        while u17 < n_groups {
                            let p4 = symt + u17 * 4;
                            let w = self.r16s(self.r32(self.r32(p4)));
                            acc = (acc - w) & 0xffff;
                            let gcnt17 = self.r16u(gc + u17 * 2);
                            let wn = self.r16u(self.r32(self.r32(p4).wrapping_add(4)));
                            if gcnt17 > 1 && ((acc & 0x8000) != 0 || (acc as u32) < wn) {
                                found = true;
                                break;
                            }
                            u17 += 1;
                        }
                        if !found {
                            la += 1;
                        } else {
                            let v = self.r16s(pgc);
                            self.w16(pgc, v - 1);
                            moved += 1;
                            la += 1;
                            let v = self.r32(p16);
                            self.w32(p16, v.wrapping_sub(4));
                            let v = self.r16s(gc + la * 2);
                            self.w16(gc + la * 2, v + 2);
                            let v = self.r32(p2 + 8);
                            self.w32(p2 + 8, v.wrapping_add(4));
                            if la < u17.wrapping_sub(1) & 0xffff {
                                let span = u17.wrapping_sub(1) - la;
                                la += span;
                                for _ in 0..span {
                                    let v = self.r32(p16 + 8);
                                    self.w32(p16 + 8, v.wrapping_add(4));
                                    p16 += 4;
                                }
                            }
                            let v = self.r16s(gc + la * 2);
                            self.w16(gc + la * 2, v + 1);
                            let v = self.r32(p16 + 4);
                            self.w32(p16 + 4, v.wrapping_add(4));
                            let v = self.r16s(gc + 2 + la * 2);
                            self.w16(gc + 2 + la * 2, v - 2);
                            if self.r16s(gc + ustk * 2) == 0 {
                                n_groups -= 1;
                                self.w16(0x0a, n_groups as i32);
                                ustk = ustk.wrapping_sub(1) & 0xffff;
                            }
                            la = 0;
                        }
                    } else {
                        moved += 1;
                        let v = self.r16s(pgc.wrapping_sub(2));
                        self.w16(pgc.wrapping_sub(2), v + 1);
                        let v = self.r16s(pgc);
                        self.w16(pgc, v - 3);
                        let v = self.r16s(gc + 2 + la * 2);
                        self.w16(gc + 2 + la * 2, v + 2);
                        let v = self.r32(p2);
                        self.w32(p2, v.wrapping_add(4));
                        let v = self.r32(p2 + 4);
                        self.w32(p2 + 4, v.wrapping_sub(8));
                        if ustk == la {
                            n_groups += 1;
                            self.w16(0x0a, n_groups as i32);
                            ustk = ustk.wrapping_add(1) & 0xffff;
                        }
                        la = 0;
                    }
                }
                if la >= n_groups {
                    break;
                }
            }
        }

        // Tune the periods by how much moved.
        if moved < 0x10 {
            if moved < 8 && self.r16u(0x2e) != 1 {
                let v = self.r16u(0x02);
                self.w16(0x02, (v << 1) as i32);
                let v = self.r16u(0x2c);
                self.w16(0x2c, (v >> 1) as i32);
                let v = self.r16u(0x2e);
                self.w16(0x2e, (v >> 1) as i32);
            }
        } else {
            let v = self.r16u(0x32);
            self.w16(0x02, v as i32);
            let v = self.r16u(0x30);
            self.w16(0x2e, v as i32);
        }
        let v = self.r16u(0x02);
        self.w16(0x00, v as i32);
        if self.r16u(0x2c) == 0 {
            let v = self.r16u(0x2e);
            self.w16(0x2c, v as i32);
        }
    }

    fn decode(&mut self, bi: &mut BitIn, buf: &[u8]) -> Result<u16, String> {
        if bi.pos >= bi.end {
            return Err("patch stream ends early".into());
        }
        let bl = bi.bl as i32;
        let mut val = ((1i32 << bl) - 1) & buf[bi.pos] as i32;
        let mut idx = bl - 1;
        let mut tot = bl;
        let lim = self.r32(0x10);
        if (val as u32) < self.r16u(lim + (idx as usize) * 8) {
            loop {
                bi.pos += 1;
                if bi.pos >= bi.end {
                    return Err("patch stream ends early".into());
                }
                idx += 8;
                tot += 8;
                val = (((val & 0xff) << 8) | buf[bi.pos] as i32) & 0xffff;
                if (val as u32) >= self.r16u(lim + (idx as usize) * 8) {
                    break;
                }
            }
        }
        idx -= 1;
        let mut cnt = (tot - 1) & 0xff;
        let mut unused: i32 = 0;
        while cnt != 0 {
            let lim_off = lim.wrapping_add(2).wrapping_add((idx as usize).wrapping_mul(8));
            let threshold = if lim_off < self.m.len().saturating_sub(1) { self.r16u(lim_off) as i32 } else { 0 };
            if val < threshold {
                break;
            }
            idx -= 1;
            cnt -= 1;
            val >>= 1;
            unused += 1;
        }
        let symt = self.r32(0x20);
        let level = ((idx + 1) & 0xffff) as usize;
        let slot_arr = self.r32(symt + level * 4);
        let k = (val - self.r16s(lim + level * 8)) as usize & 0xffff;
        let p = self.r32(slot_arr + k * 4);
        let sym = ((p.wrapping_sub(self.r32(0x18))) >> 1) as u16;
        if unused == 0 {
            unused = 8;
            bi.pos += 1;
        }
        bi.bl = unused as u32;
        if self.update_freq(sym as usize) {
            self.rebuild();
            self.build_limits(0);
        }
        if sym == self.r16u(0x24) as u16 {
            // A symbol not seen before, sent as it is.
            let raw = bi.bits(self.r16u(0x06), buf)?;
            let level = self.add_symbol(raw as usize);
            self.build_limits(level);
            return Ok(raw as u16);
        }
        Ok(sym)
    }
}

/// Decodes the stream at `offset`, up to `limit` bytes of output.
fn decompress(data: &[u8], offset: usize, limit: usize) -> Result<Vec<u8>, String> {
    let mut bi = BitIn { pos: offset, end: data.len(), bl: 8 };
    if bi.bits(16, data)? != STREAM_MAGIC {
        return Err("the patch isn't the expected one".into());
    }
    let raw_literals = bi.bits(8, data)? != 0;
    bi.bits(8, data)?;
    let init_period = bi.bits(12, data)?;
    let upd_period = bi.bits(12, data)?;
    let dist_bits = if bi.bits(4, data)? == 8 { 7 } else { 6 };
    let mut literals = (!raw_literals).then(|| HuffTree::new(8, 0x10, init_period, upd_period));
    let mut lengths = HuffTree::new(6, 0x0c, init_period, upd_period);
    let mut distances = HuffTree::new(6, 0x0c, init_period, upd_period);
    let mut out = Vec::with_capacity(limit.min(1 << 21));
    while out.len() < limit {
        if bi.bit(data)? == 0 {
            let byte = match literals.as_mut() {
                Some(tree) => tree.decode(&mut bi, data)?,
                None => bi.bits(8, data)? as u16,
            };
            out.push(byte as u8);
        } else {
            let low = bi.bits(dist_bits, data)?;
            let high = distances.decode(&mut bi, data)? as u32;
            let distance = (high << dist_bits) | low;
            if distance == 0 {
                break;
            }
            let length = (lengths.decode(&mut bi, data)? & 0x7f) as usize;
            let back = distance as usize + 1;
            for _ in 0..length {
                if out.len() >= limit {
                    break;
                }
                let at = out.len();
                out.push(if at >= back { out[at - back] } else { 0 });
            }
        }
    }
    Ok(out)
}

// A changed file's steps (rtptool's `apply.rs`), for one old file.

struct Steps<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Steps<'a> {
    fn byte(&mut self) -> Result<u8, String> {
        let byte = *self.data.get(self.pos).ok_or("patch steps end early")?;
        self.pos += 1;
        Ok(byte)
    }

    fn bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
        let bytes = self.data.get(self.pos..self.pos + n).ok_or("patch steps end early")?;
        self.pos += n;
        Ok(bytes)
    }

    /// A number: the top bit is the sign, the run of ones after it says how
    /// many bytes follow (low byte first), and the rest are its top bits.
    fn number(&mut self) -> Result<i64, String> {
        let first = self.byte()? as u64;
        let mut count = 0u32;
        let mut mask = 0x40u64;
        while mask > 0 && (first & mask) != 0 {
            count += 1;
            mask >>= 1;
        }
        let value = if count == 0 {
            first & 0x3f
        } else {
            let mut tail = 0u64;
            for i in 0..count {
                tail |= (self.byte()? as u64) << (8 * i);
            }
            ((first & ((0x40u64 >> count) - 1)) << (8 * count)) | tail
        };
        Ok(if first & 0x80 != 0 { -(value as i64) } else { value as i64 })
    }

    fn size(&mut self) -> Result<usize, String> {
        usize::try_from(self.number()?).map_err(|_| "bad patch step".to_string())
    }
}

fn apply(old: &[u8], steps: &[u8], size: usize) -> Result<Vec<u8>, String> {
    let mut r = Steps { data: steps, pos: 0 };
    let mut out = vec![0u8; size];
    let mut at: usize = 0;
    let mut poke_at: i64 = 0;
    let mut gaps: Vec<(usize, usize)> = Vec::new();
    let mut templates: Vec<(usize, usize)> = Vec::new();

    fn grow(out: &mut Vec<u8>, n: usize) {
        if n > out.len() {
            out.resize(n, 0);
        }
    }
    fn copy(old: &[u8], out: &mut Vec<u8>, at: &mut usize, gaps: &mut Vec<(usize, usize)>, advance: usize, from: usize, count: usize) {
        if advance > 0 {
            gaps.push((*at, advance));
            *at += advance;
        }
        grow(out, *at + count);
        let end = (from + count).min(old.len());
        let length = end.saturating_sub(from);
        if length > 0 {
            out[*at..*at + length].copy_from_slice(&old[from..end]);
        }
        *at += count;
    }
    fn fill(out: &mut Vec<u8>, at: &mut usize, gaps: &mut Vec<(usize, usize)>, seek: usize, pattern: &[u8], count: usize) {
        if seek > 0 {
            gaps.push((*at, seek));
            *at += seek;
        }
        grow(out, *at + count);
        if pattern.iter().any(|&byte| byte != 0) {
            for i in 0..count {
                out[*at + i] = pattern[i % pattern.len()];
            }
        }
        *at += count;
    }
    fn poke(out: &mut Vec<u8>, at: i64, width: usize, delta: i64) {
        let Ok(p) = usize::try_from(at) else { return };
        grow(out, p + width);
        let mut bytes = [0u8; 8];
        bytes[..width].copy_from_slice(&out[p..p + width]);
        let value = (u64::from_le_bytes(bytes) as i64).wrapping_add(delta);
        out[p..p + width].copy_from_slice(&value.to_le_bytes()[..width]);
    }
    let signed = |byte: u8| byte as i8 as i64;

    while r.pos < steps.len() {
        match r.byte()? {
            0x01 => break,
            0x02 => {
                r.number()?;
                at = 0;
                poke_at = 0;
            }
            step @ (0x03 | 0x04) => {
                let advance = if step == 0x04 { r.size()? } else { 0 };
                let from = r.size()?;
                let count = r.size()?;
                copy(old, &mut out, &mut at, &mut gaps, advance, from, count);
            }
            0x05 => {
                if size > at {
                    gaps.push((at, size - at));
                }
                for &(gap, length) in &gaps {
                    grow(&mut out, gap + length);
                    out[gap..gap + length].copy_from_slice(r.bytes(length)?);
                }
                gaps.clear();
            }
            0x06 => {
                poke_at += r.number()?;
                let delta = signed(r.byte()?);
                poke(&mut out, poke_at, 1, delta);
            }
            0x07 | 0x0e => {
                poke_at = 0;
                let delta = signed(r.byte()?);
                for _ in 0..r.size()? {
                    poke_at += r.number()?;
                    poke(&mut out, poke_at, 1, delta);
                }
            }
            0x08 => {
                let from = r.size()?;
                let count = r.size()?;
                templates.push((from, count));
            }
            step @ (0x09 | 0x0a) => {
                let advance = if step == 0x0a { r.size()? } else { 0 };
                let &(from, count) = templates.get(r.size()?).ok_or("bad patch template")?;
                copy(old, &mut out, &mut at, &mut gaps, advance, from, count);
            }
            step @ (0x0b | 0x0c) => {
                let seek = if step == 0x0c { r.size()? } else { 0 };
                let count = r.size()?;
                fill(&mut out, &mut at, &mut gaps, seek, &[], count);
            }
            0x0d => {
                poke_at = 0;
                for _ in 0..r.size()? {
                    poke_at += r.number()?;
                    let delta = signed(r.byte()?);
                    poke(&mut out, poke_at, 1, delta);
                }
            }
            0x0f => {
                poke_at = 0;
                let delta = i16::from_le_bytes(r.bytes(2)?.try_into().unwrap()) as i64;
                for _ in 0..r.size()? {
                    poke_at += r.number()?;
                    poke(&mut out, poke_at, 2, delta);
                }
            }
            0x10 => {
                poke_at = 0;
                let delta = i32::from_le_bytes(r.bytes(4)?.try_into().unwrap()) as i64;
                for _ in 0..r.size()? {
                    poke_at += r.number()?;
                    poke(&mut out, poke_at, 4, delta);
                }
            }
            step @ (0x11..=0x16) => {
                let seek = if step >= 0x14 { r.size()? } else { 0 };
                let width = [1, 2, 4][((step - 0x11) % 3) as usize];
                let pattern = r.bytes(width)?.to_vec();
                let count = r.size()?;
                fill(&mut out, &mut at, &mut gaps, seek, &pattern, count);
            }
            other => return Err(format!("unknown patch step {other:#x}")),
        }
    }
    out.truncate(size);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inflates_stored_and_fixed_blocks() {
        // "hello" stored, and "hello hello hello" with fixed codes (Python's zlib, raw deflate).
        let stored = [1u8, 5, 0, 0xfa, 0xff, b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(inflate(&stored, 5).unwrap(), b"hello");
        let fixed = [0xcbu8, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0xc8, 0x40, 0x90, 0x00];
        assert_eq!(inflate(&fixed, 17).unwrap(), b"hello hello hello");
    }

    #[test]
    fn step_numbers() {
        let mut steps = Steps { data: &[0x05, 0x85, 0x41, 0x34, 0x12], pos: 0 };
        assert_eq!(steps.number().unwrap(), 5);
        assert_eq!(steps.number().unwrap(), -5);
        // One byte follows: 0x01 << 8 | 0x34... with the top bits of 0x41 (one length bit, value 1).
        assert_eq!(steps.number().unwrap(), 0x0134);
    }

    #[test]
    fn steps_copy_and_fill_gaps() {
        // Copy 3 bytes from 2 after a gap of 2, then fill the gap and the tail.
        let old = b"abcdefgh";
        let steps = [0x04, 2, 2, 3, 0x05, b'X', b'Y', b'Z', 0x01];
        assert_eq!(apply(old, &steps, 6).unwrap(), b"XYcdeZ");
    }

    #[test]
    fn a_wrong_file_is_not_the_patch() {
        let dir = std::env::temp_dir().join("rtpatch-test-not-a-patch");
        std::fs::write(&dir, b"PK\x03\x04 not really").unwrap();
        assert!(patch_program(&dir).is_none());
        let _ = std::fs::remove_file(&dir);
    }
}
