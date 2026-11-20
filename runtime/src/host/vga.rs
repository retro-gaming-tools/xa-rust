//! The VGA and a VESA BIOS, for recompiled DOS programs.
//!
//! DOS games drew straight into video memory: 320x200 in mode 13h at
//! 0xA0000, or, through a VESA BIOS (VBE 2.0), 640x480 and more in a linear
//! frame buffer. Colours came from the VGA's palette (its DAC), 6 bits per
//! component, set through ports 0x3C8 and 0x3C9 or VBE function 9.
//!
//! The runtime offers VBE 2.0 with a linear frame buffer, 640x400 and
//! 640x480 in 256 colours (the mode Z's VESA option uses), 320x200 and
//! 640x480 in 15, 16, 24 and 32 bits (see `MODES`), and mode 13h. What the
//! program draws there is shown in the window at every refresh, as a
//! monitor did (see `frame`).

use crate::cpu::Cpu;

/// Where the frame buffer seems to be, for DPMI's physical mapping, and how
/// much video memory there is.
const LFB_PHYSICAL: u32 = 0xe000_0000;
const VIDEO_MEMORY: u32 = 4 << 20;
const MODE_13H: u16 = 0x13;
const TEXT_MODE: u16 = 0x03;
/// The VESA modes offered: the standard numbers up to 640x480, in 256
/// colours, 32768 and 65536 colours and true colour, with their bits per
/// pixel. A game offers the player the ones it can draw: Extreme Assault's
/// software renderer has 8-bit and hicolor and truecolor paths. Bigger modes
/// don't fit the window's 640x480 picture.
const MODES: [(u16, u32, u32, u32); 10] = [
    (0x100, 640, 400, 8),
    (0x101, 640, 480, 8),
    (0x10d, 320, 200, 15),
    (0x10e, 320, 200, 16),
    (0x10f, 320, 200, 24),
    (0x110, 640, 480, 15),
    (0x111, 640, 480, 16),
    (0x112, 640, 480, 24),
    (0x120, 320, 200, 32),
    (0x121, 640, 480, 32),
];
const VBE_OK: u32 = 0x004f;
const VBE_FAILED: u32 = 0x014f;
const LINEAR: u16 = 0x4000;
const WINDOW: u32 = 0xa0000;
const WINDOW_SIZE: u32 = 0x10000;

pub struct Vga {
    pub mode: u16,
    pub width: u32,
    pub height: u32,
    /// Bits per pixel: 8 with the palette, or 15, 16, 24 or 32 direct colour.
    pub bits: u32,
    /// Where the program draws: the frame buffer's linear address.
    pub frame_buffer: u32,
    /// The linear frame buffer, once mapped.
    linear: u32,
    /// The 64 KB bank shown at 0xA0000, for banked VESA modes.
    bank: u32,
    /// Whether the program has switched banks since setting its mode. Only
    /// then is the window its picture; a program drawing in the linear frame
    /// buffer would lose its top 64 KB to the empty window otherwise.
    banked: bool,
    pub palette: [[u8; 3]; 256],
    write_index: usize,
    read_index: usize,
    component: usize,
    /// Where the picture starts in video memory, for scrolling and page flips.
    display_start: u32,
}

impl Default for Vga {
    fn default() -> Vga {
        // The VGA's default colours are close enough: grey steps.
        let mut palette = [[0u8; 3]; 256];
        for (index, colour) in palette.iter_mut().enumerate() {
            *colour = [(index >> 2) as u8; 3];
        }
        Vga { mode: TEXT_MODE, width: 0, height: 0, bits: 8, frame_buffer: 0, linear: 0, bank: 0, banked: false, palette, write_index: 0, read_index: 0, component: 0, display_start: 0 }
    }
}

/// The registers a video BIOS call sees: the program's own, or those of a
/// real-mode call through DPMI. `es` is a real-mode segment for the latter,
/// and buffers are at `es * 16 + di` then.
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub edi: u32,
    /// Where a buffer given as ES:DI is.
    pub buffer: u32,
    /// A buffer's real-mode segment, for far pointers written into it.
    pub segment: u32,
}

/// A VESA mode's width, height and bits per pixel.
fn mode_size(mode: u16) -> Option<(u32, u32, u32)> {
    MODES.iter().find(|(number, ..)| *number == mode & 0x1ff).map(|&(_, w, h, bits)| (w, h, bits))
}

/// Bytes a pixel takes: 15-bit colour is stored in two.
fn bytes_per_pixel(bits: u32) -> u32 {
    bits.div_ceil(8)
}

/// Bytes a scan line of the current mode takes.
fn pitch(vga: &Vga) -> u32 {
    vga.width * bytes_per_pixel(vga.bits)
}

/// Maps the linear frame buffer, as DPMI 0x0800 does: guest memory for it,
/// once. Other physical addresses (the first megabyte) are where they are.
pub fn map_physical(cpu: &mut Cpu, physical: u32) -> u32 {
    if !(LFB_PHYSICAL..LFB_PHYSICAL + VIDEO_MEMORY).contains(&physical) {
        return physical;
    }
    if cpu.host.vga.linear == 0 {
        let memory = cpu.allocate(VIDEO_MEMORY);
        cpu.fill(memory, VIDEO_MEMORY, 0);
        cpu.host.vga.linear = memory;
    }
    cpu.host.vga.linear + (physical - LFB_PHYSICAL)
}

fn set_mode(cpu: &mut Cpu, mode: u16) -> bool {
    let vga = &mut cpu.host.vga;
    if mode & 0xff == MODE_13H && mode < 0x100 {
        vga.mode = MODE_13H;
        (vga.width, vga.height, vga.bits, vga.frame_buffer) = (320, 200, 8, WINDOW);
        cpu.fill(WINDOW, 320 * 200, 0);
    } else if mode & 0x7f == TEXT_MODE && mode < 0x100 {
        vga.mode = TEXT_MODE;
        (vga.width, vga.height) = (0, 0);
    } else if let Some((width, height, bits)) = mode_size(mode) {
        vga.mode = mode & 0x1ff;
        (vga.width, vga.height, vga.bits) = (width, height, bits);
        println!("VESA mode {:#x}: {width}x{height}, {bits} bits a pixel", mode & 0x1ff);
        vga.display_start = 0;
        vga.bank = 0;
        vga.banked = false;
        let linear = map_physical(cpu, LFB_PHYSICAL);
        cpu.host.vga.frame_buffer = linear;
        if mode & 0x8000 == 0 {
            cpu.fill(linear, VIDEO_MEMORY, 0);
        }
    } else {
        return false;
    }
    true
}

/// `int 10h`, with the registers in `r`. Returns false for a function the
/// runtime doesn't have.
pub fn bios(cpu: &mut Cpu, r: &mut Registers) -> bool {
    let (ah, ax) = ((r.eax >> 8) & 0xff, r.eax & 0xffff);
    let set_ax = |r: &mut Registers, value: u32| r.eax = (r.eax & !0xffff) | value;
    match (ah, ax) {
        (0x00, _) => {
            set_mode(cpu, (r.eax & 0xff) as u16);
        }
        // The current mode, and 80 columns.
        (0x0f, _) => r.eax = (r.eax & !0xffff) | 80 << 8 | (cpu.host.vga.mode & 0xff) as u32,
        // The cursor and text: nothing to show in graphics.
        (0x01 | 0x02 | 0x03 | 0x06 | 0x09 | 0x0e, _) => {}
        // Palette registers: block of DAC colours from ES:DX.
        (0x10, _) if r.eax & 0xff == 0x12 => {
            let (first, count) = ((r.ebx & 0xffff) as usize, (r.ecx & 0xffff) as usize);
            for index in 0..count.min(256 - first.min(256)) {
                let at = r.buffer + index as u32 * 3;
                cpu.host.vga.palette[first + index] = [cpu.r8(at) & 0x3f, cpu.r8(at + 1) & 0x3f, cpu.r8(at + 2) & 0x3f];
            }
        }
        (0x10, _) => {}
        // VBE: the controller.
        (_, 0x4f00) => {
            let at = r.buffer;
            cpu.fill(at, 512, 0);
            cpu.bytes_mut(at, 4).copy_from_slice(b"VESA");
            cpu.w16(at + 4, 0x0200);
            // The OEM name and the mode list are inside the block, as far
            // pointers to it.
            let far = |offset: u32| (r.segment << 16) | ((r.buffer - r.segment * 16) + offset);
            cpu.write_string(at + 0x100, "Z-Rust VBE");
            cpu.w32(at + 6, far(0x100));
            cpu.w32(at + 0x0a, 0);
            cpu.w32(at + 0x0e, far(0x120));
            for (index, (mode, ..)) in MODES.iter().enumerate() {
                cpu.w16(at + 0x120 + index as u32 * 2, *mode);
            }
            cpu.w16(at + 0x120 + MODES.len() as u32 * 2, 0xffff);
            cpu.w16(at + 0x12, (VIDEO_MEMORY >> 16) as u16);
            set_ax(r, VBE_OK);
        }
        // VBE: a mode.
        (_, 0x4f01) => {
            let Some((width, height, bits)) = mode_size((r.ecx & 0xffff) as u16) else {
                set_ax(r, VBE_FAILED);
                return true;
            };
            let at = r.buffer;
            let line = width * bytes_per_pixel(bits);
            cpu.fill(at, 256, 0);
            // Supported, colour, graphics, linear frame buffer available.
            cpu.w16(at, 0x009b);
            cpu.w8(at + 2, 7);
            cpu.w16(at + 4, 64);
            cpu.w16(at + 6, 64);
            cpu.w16(at + 8, (WINDOW >> 4) as u16);
            cpu.w16(at + 0x10, line as u16);
            cpu.w16(at + 0x12, width as u16);
            cpu.w16(at + 0x14, height as u16);
            cpu.w8(at + 0x16, 8);
            cpu.w8(at + 0x17, 16);
            cpu.w8(at + 0x18, 1);
            cpu.w8(at + 0x19, bits as u8);
            cpu.w8(at + 0x1a, 1);
            // Packed pixels with the palette, or direct colour, with each
            // component's size and place: 5:5:5, 5:6:5 or 8:8:8.
            cpu.w8(at + 0x1b, if bits == 8 { 4 } else { 6 });
            cpu.w8(at + 0x1d, (VIDEO_MEMORY / (line * height)).saturating_sub(1) as u8);
            cpu.w8(at + 0x1e, 1);
            let masks: [u8; 8] = match bits {
                15 => [5, 10, 5, 5, 5, 0, 1, 15],
                16 => [5, 11, 6, 5, 5, 0, 0, 0],
                24 => [8, 16, 8, 8, 8, 0, 0, 0],
                32 => [8, 16, 8, 8, 8, 0, 8, 24],
                _ => [0; 8],
            };
            cpu.bytes_mut(at + 0x1f, 8).copy_from_slice(&masks);
            cpu.w32(at + 0x28, LFB_PHYSICAL);
            set_ax(r, VBE_OK);
        }
        (_, 0x4f02) => {
            let mode = (r.ebx & 0xffff) as u16;
            let ok = set_mode(cpu, mode & !LINEAR) || set_mode(cpu, mode);
            set_ax(r, if ok { VBE_OK } else { VBE_FAILED });
        }
        (_, 0x4f03) => {
            r.ebx = (r.ebx & !0xffff) | cpu.host.vga.mode as u32;
            set_ax(r, VBE_OK);
        }
        // VBE: the bank shown in the window at 0xA0000. Banked drawing goes
        // to the window, and is copied into the frame buffer on each switch.
        (_, 0x4f05) => {
            if r.ebx & 0xff00 == 0 {
                flush_bank(cpu);
                cpu.host.vga.banked = true;
                cpu.host.vga.bank = r.edx & 0xffff;
                let source = cpu.host.vga.frame_buffer + cpu.host.vga.bank * WINDOW_SIZE;
                let data = cpu.bytes(source, WINDOW_SIZE).to_vec();
                cpu.bytes_mut(WINDOW, WINDOW_SIZE).copy_from_slice(&data);
            } else {
                r.edx = (r.edx & !0xffff) | cpu.host.vga.bank;
            }
            set_ax(r, VBE_OK);
        }
        // VBE: bytes a scan line, and where the picture starts.
        (_, 0x4f06) => {
            let (width, line) = (cpu.host.vga.width, pitch(&cpu.host.vga));
            r.ebx = (r.ebx & !0xffff) | line;
            r.ecx = (r.ecx & !0xffff) | width;
            r.edx = (r.edx & !0xffff) | (VIDEO_MEMORY / line.max(1)).min(0xffff);
            set_ax(r, VBE_OK);
        }
        // Where the picture starts, in pixels and lines; kept in bytes.
        (_, 0x4f07) => {
            let (line, size) = (pitch(&cpu.host.vga).max(1), bytes_per_pixel(cpu.host.vga.bits));
            if r.ebx & 0xff == 0 || r.ebx & 0xff == 0x80 {
                cpu.host.vga.display_start = (r.edx & 0xffff) * line + (r.ecx & 0xffff) * size;
            } else {
                r.ecx = (r.ecx & !0xffff) | cpu.host.vga.display_start % line / size;
                r.edx = (r.edx & !0xffff) | cpu.host.vga.display_start / line;
            }
            set_ax(r, VBE_OK);
        }
        // VBE: the DAC stays 6 bits wide.
        (_, 0x4f08) => {
            r.ebx = (r.ebx & !0xff00) | 6 << 8;
            set_ax(r, VBE_OK);
        }
        // VBE: palette entries from ES:DI, blue, green, red, padding.
        (_, 0x4f09) => {
            if r.ebx & 0xff == 0 || r.ebx & 0xff == 0x80 {
                let (count, first) = ((r.ecx & 0xffff) as usize, (r.edx & 0xffff) as usize);
                for index in 0..count.min(256 - first.min(256)) {
                    let at = r.buffer + index as u32 * 4;
                    cpu.host.vga.palette[first + index] = [cpu.r8(at + 2) & 0x3f, cpu.r8(at + 1) & 0x3f, cpu.r8(at) & 0x3f];
                }
            }
            set_ax(r, VBE_OK);
        }
        // VBE: no protected-mode interface; the program calls the BIOS.
        (_, 0x4f0a) => set_ax(r, VBE_FAILED),
        _ => return false,
    }
    true
}

/// Banked drawing: what was drawn in the window goes back to its bank.
fn flush_bank(cpu: &mut Cpu) {
    let vga = &cpu.host.vga;
    if vga.mode < 0x100 || vga.frame_buffer == 0 || !vga.banked {
        return;
    }
    let target = vga.frame_buffer + vga.bank * WINDOW_SIZE;
    let data = cpu.bytes(WINDOW, WINDOW_SIZE).to_vec();
    cpu.bytes_mut(target, WINDOW_SIZE).copy_from_slice(&data);
}

/// The DAC's ports: 0x3C7 picks a colour to read, 0x3C8 one to write, and
/// 0x3C9 reads or writes its red, green and blue in turn.
pub fn read(cpu: &mut Cpu, port: u16) -> Option<u32> {
    let vga = &mut cpu.host.vga;
    Some(match port {
        0x3c9 => {
            let value = vga.palette[vga.read_index & 0xff][vga.component];
            vga.component += 1;
            if vga.component == 3 {
                vga.component = 0;
                vga.read_index += 1;
            }
            value as u32
        }
        0x3c8 => vga.write_index as u32 & 0xff,
        _ => return None,
    })
}

pub fn write(cpu: &mut Cpu, port: u16, value: u32) -> bool {
    let vga = &mut cpu.host.vga;
    match port {
        0x3c7 => {
            vga.read_index = (value & 0xff) as usize;
            vga.component = 0;
        }
        0x3c8 => {
            vga.write_index = (value & 0xff) as usize;
            vga.component = 0;
        }
        0x3c9 => {
            vga.palette[vga.write_index & 0xff][vga.component] = (value & 0x3f) as u8;
            vga.component += 1;
            if vga.component == 3 {
                vga.component = 0;
                vga.write_index += 1;
            }
        }
        // Other VGA registers: sequencer, CRT and attribute controllers,
        // which a game sets for its mode. They change nothing here.
        0x3c0..=0x3cf | 0x3d4 | 0x3d5 => {}
        _ => return false,
    }
    true
}

/// The picture for the window, as 640x480 16-bit pixels, from what the
/// program drew and the palette: `None` in text mode. 320x200 is doubled,
/// and a smaller picture is centred.
pub fn frame(cpu: &mut Cpu) -> Option<Vec<u16>> {
    const WIDTH: u32 = super::GAME_WIDTH as u32;
    const HEIGHT: u32 = super::GAME_HEIGHT as u32;
    if cpu.host.vga.mode == TEXT_MODE || cpu.host.vga.width == 0 {
        return None;
    }
    flush_bank(cpu);
    let vga = &cpu.host.vga;
    let table: Vec<u16> = vga.palette.iter().map(|c| ((c[0] as u16) >> 1) << 11 | (c[1] as u16) << 5 | (c[2] as u16) >> 1).collect();
    let scale = if vga.width * 2 <= WIDTH && vga.height * 2 <= HEIGHT { 2 } else { 1 };
    let (shown_width, shown_height) = ((vga.width * scale).min(WIDTH), (vga.height * scale).min(HEIGHT));
    let (left, top) = ((WIDTH - shown_width) / 2, (HEIGHT - shown_height) / 2);
    let source = vga.frame_buffer + vga.display_start;
    let (line, size) = (pitch(vga) as usize, bytes_per_pixel(vga.bits) as usize);
    let pixels = cpu.bytes(source, line as u32 * vga.height);
    let mut picture = vec![0u16; (WIDTH * HEIGHT) as usize];
    for y in 0..shown_height {
        let row = &pixels[(y / scale) as usize * line..][..line];
        let target = &mut picture[((top + y) * WIDTH + left) as usize..][..shown_width as usize];
        for (x, pixel) in target.iter_mut().enumerate() {
            let at = x / scale as usize * size;
            *pixel = match vga.bits {
                8 => table[row[at] as usize],
                15 => {
                    let p = u16::from_le_bytes([row[at], row[at + 1]]);
                    (p & 0x7fe0) << 1 | (p & 0x0200) >> 4 | p & 0x1f
                }
                16 => u16::from_le_bytes([row[at], row[at + 1]]),
                // 24 and 32 bits: blue, green, red (and a spare byte).
                _ => (row[at + 2] as u16 >> 3) << 11 | (row[at + 1] as u16 >> 2) << 5 | row[at] as u16 >> 3,
            };
        }
    }
    Some(picture)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_sizes_and_pitches() {
        assert_eq!(mode_size(0x111), Some((640, 480, 16)));
        assert_eq!(mode_size(0x4110), Some((640, 480, 15)));
        assert_eq!(mode_size(0x105), None);
        assert_eq!(bytes_per_pixel(15), 2);
        assert_eq!(bytes_per_pixel(24), 3);
        let vga = Vga { mode: 0x112, width: 640, height: 480, bits: 24, ..Vga::default() };
        assert_eq!(pitch(&vga), 1920);
    }
}
