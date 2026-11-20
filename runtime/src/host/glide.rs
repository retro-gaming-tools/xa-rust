//! Glide 2, the 3dfx API, drawn by the graphics card through OpenGL, or by a
//! software rasterizer where there's no window or no usable OpenGL.
//!
//! The game addresses a 640x480 card. Triangles are scaled up and drawn at
//! `scale` times that, so 3D is sharper than the card could manage, while 2D
//! from the linear frame buffer is scaled up by repeating pixels. A function the
//! game never calls stays a stub, sized from its decorated name.
//!
//! This file holds what both renderers share: the calls, the state, and the
//! rules for the frame buffer. `glide_gl` draws on the card and `glide_raster`
//! on the processor.

use super::glide_gl::Gpu;
use super::glide_raster::{self as raster, Batch, Combine, Managed, Screen, State, Vertex};
use crate::cpu::{Cpu, GuestFn};
use std::sync::Mutex;

use super::{GAME_HEIGHT as HEIGHT, GAME_WIDTH as WIDTH};
/// Marks 2D pixels the game didn't write, so 3D shows through them.
const UNWRITTEN: u16 = 0xf81f;
const DEFAULT_SCALE: usize = 3;
const TEXTURE_MEMORY: u32 = 4 << 20;

pub struct Glide {
    pub scale: usize,
    pub width: usize,
    pub height: usize,
    /// The graphics card's renderer. With it, the buffers below stay empty.
    gpu: Option<Gpu>,
    pub buffers: [Vec<u32>; 2],
    pub back: usize,
    pub depth: Vec<f32>,
    pub lfb: u32,
    /// What a lock of the aux (depth) buffer reads, kept apart from `lfb`:
    /// Extreme Assault 1.1 reads it three times a frame, for depth tests of
    /// single points, while its write lock of the 2D stays held in `lfb`.
    /// Sharing the memory put a 640x480 copy of the 3D into the 2D, and each
    /// swap drew it over the scene, so missions showed at 640x480 at any
    /// resolution. It reads as the farthest depth everywhere: the runtime's
    /// depth isn't the Voodoo's format, so the game's tests (lights and the
    /// sun) always find their points unhidden.
    aux_lfb: u32,
    /// A buffer the game locked for writing and hasn't unlocked. Extreme
    /// Assault's first versions lock the back buffer once and swap with it
    /// still locked: the card showed whatever was in its memory, so each swap
    /// takes in what the game wrote since.
    held: Option<u32>,
    /// A write lock through the pixel pipeline (see `Pipeline`).
    pipeline: Option<Pipeline>,
    /// Pixels of 2D blended rather than copied: this frame's so far, and the
    /// last whole frame's, for the script's `blended:` check.
    blended: (u64, u64),
    /// The same for 2D copied, for the script's `copied:` check.
    copied: (u64, u64),
    pub frames: u64,
    /// Counts swaps and writes to the front buffer: each is a new picture to show.
    pub updates: u64,
    /// True once this frame has drawn triangles or cleared the buffer.
    drew_3d: bool,
    /// For the `modes` trace: triangles drawn this frame by their settings.
    mode_tally: Option<std::collections::BTreeMap<String, u64>>,
    /// While frames draw no 3D: the last 3D frame, without its pointer. Each
    /// such frame starts from it, as a 3D frame starts from a fresh render.
    /// Incubation's load and save window draws only itself and the pointer
    /// each frame, so without this the pointer left a trail outside it.
    scene: Option<Vec<u32>>,
    /// The same for the graphics card, which keeps the scene itself.
    scene_on_card: bool,
    /// True once this frame has started from `scene`.
    restored: bool,
    /// The pixels around the pointer just before the latest 2D write of a 3D
    /// frame, which come before the pointer is drawn: x, y, width, pixels.
    under_pointer: (usize, usize, usize, Vec<u32>),
    /// The same area on the graphics card, which keeps the pixels itself: x,
    /// y, width and height.
    under_pointer_on_card: Option<[i32; 4]>,
    pub state: State,
    batch: Batch,
    pool: super::pool::Pool,
    /// For the script's `fps` step: seconds drawing, seconds on 2D, and 2D locks.
    pub spent: [f64; 5],
    last_swap: std::time::Instant,
    /// The longest frame since the `fps` step last read it, in seconds.
    pub longest_frame: f64,
}

/// A blending mode of pixel-pipeline writes: Glide's source and destination
/// factors, and whether the colour combine inverts the written colour.
type Mode = ([u32; 2], bool);

/// The pixels of a write lock through the pixel pipeline. The card blended
/// each pixel as it was written, with the blending then set. Extreme
/// Assault's Glide program writes its 2D this way, changing the blending
/// about 40 times a frame: the cockpit frame copied, its shadow, the text
/// boxes and the radar multiplied over the 3D, and text and blips added. Each
/// change of mode records which pixels were written since, under which mode,
/// and the lock's end takes them in with one pass for each mode. A pixel
/// written twice keeps its last write, as where text lies on a box.
struct Pipeline {
    /// The frame buffer as it was at the last change of mode, `WIDTH` to a row.
    before: Vec<u16>,
    /// Each pixel's last write not yet taken in, or `UNWRITTEN`.
    values: Vec<u16>,
    /// The mode each of those was written under: an index into `modes`.
    mode_of: Vec<u8>,
    /// The modes met since the last taking in, in the order met.
    modes: Vec<Mode>,
    /// The mode now in force.
    current: u8,
    /// The rows holding writes not yet taken in: first, and one past the last.
    /// Mission frames change modes about 20 times while writing a few strips
    /// of HUD, so only these rows are gone through.
    rows: (usize, usize),
}

impl Pipeline {
    fn new(mode: Mode) -> Pipeline {
        Pipeline { before: vec![UNWRITTEN; WIDTH * HEIGHT], values: vec![UNWRITTEN; WIDTH * HEIGHT], mode_of: vec![0; WIDTH * HEIGHT], modes: vec![mode], current: 0, rows: (HEIGHT, 0) }
    }

    /// After the writes were taken in: nothing waits, in the mode now in force.
    fn taken_in(&mut self) {
        let (first, end) = self.rows;
        if first < end {
            self.values[first * WIDTH..end * WIDTH].fill(UNWRITTEN);
        }
        self.rows = (HEIGHT, 0);
        self.modes = vec![self.modes[self.current as usize]];
        self.current = 0;
    }
}

impl Glide {
    fn new(cpu: &mut Cpu) -> Glide {
        // The loader sets this from the launcher, `--scale=N`, or the player's own.
        let scale = std::env::var("RECOMP_SCALE").ok().and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_SCALE).clamp(1, 6);
        let (width, height) = (WIDTH * scale, HEIGHT * scale);
        let lfb = cpu.allocate((LFB_STRIDE * HEIGHT * 2) as u32);
        let aux_lfb = cpu.allocate((LFB_STRIDE * HEIGHT * 2) as u32);
        // The farthest depth everywhere, 0xffff in each pixel.
        cpu.fill(aux_lfb, (LFB_STRIDE * HEIGHT * 2) as u32, 0xff);
        let mut state = State::default();
        state.clip = [0, 0, WIDTH as i32, HEIGHT as i32];
        // The window has OpenGL unless the player chose the processor or the
        // machine has none. If the card can't be set up, the processor draws.
        let gpu = cpu.host.video.as_ref().and_then(|video| video.gl()).and_then(|gl| match Gpu::new(gl, scale) {
            Ok(gpu) => Some(gpu),
            Err(problem) => {
                eprintln!("Glide is drawn by the processor: {problem}");
                None
            }
        });
        println!("Glide at {width}x{height}, drawn by the {}", if gpu.is_some() { "graphics card" } else { "processor" });
        let pixels = if gpu.is_some() { 0 } else { width * height };
        Glide {
            scale,
            width,
            height,
            gpu,
            buffers: [vec![0; pixels], vec![0; pixels]],
            back: 0,
            depth: vec![0.0; pixels],
            lfb,
            aux_lfb,
            held: None,
            pipeline: None,
            blended: (0, 0),
            copied: (0, 0),
            frames: 0,
            updates: 0,
            drew_3d: false,
            scene: None,
            scene_on_card: false,
            restored: false,
            under_pointer: (0, 0, 0, Vec::new()),
            under_pointer_on_card: None,
            state,
            batch: Batch::default(),
            pool: super::pool::Pool::new(
                std::env::var("RECOMP_THREADS").ok().and_then(|s| s.parse().ok())
                    .or(std::thread::available_parallelism().ok().map(usize::from))
                    .unwrap_or(1),
            ),
            spent: [0.0; 5],
            last_swap: std::time::Instant::now(),
            longest_frame: 0.0,
            mode_tally: None,
        }
    }

    fn draw(&mut self, vertices: [Vertex; 3]) {
        if let Some(tally) = self.mode_tally.as_mut() {
            let s = &self.state;
            let key = format!(
                "blend {:?} color {}/{}/{}/{} alpha {}/{}/{}/{} constant {:#x} depth {}/{} write {}/{} clip {:?}",
                s.blend, s.color.function, s.color.factor, s.color.local, s.color.other, s.alpha.function, s.alpha.factor, s.alpha.local, s.alpha.other,
                s.constant, s.depth_mode, s.depth_function, s.color_write, s.depth_write, s.clip
            );
            *tally.entry(key).or_insert(0) += 1;
        }
        let vertices = if self.state.blend == [4, 0] && self.state.depth_mode != 0 { widen(vertices, WIDEN / self.scale as f32) } else { vertices };
        self.batch.push(&self.state, vertices);
    }

    /// Draws the queued triangles. This comes before anything that touches the
    /// buffers another way.
    fn flush(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.drew_3d = true;
        let started = std::time::Instant::now();
        if let Some(gpu) = self.gpu.as_mut() {
            let (snapshots, triangles) = self.batch.take();
            gpu.draw(self.back, &snapshots, &triangles);
        } else {
            self.batch.draw(Screen {
                color: &mut self.buffers[self.back],
                depth: &mut self.depth,
                width: self.width,
                height: self.height,
                scale: self.scale,
                pool: &self.pool,
            });
        }
        self.spent[0] += started.elapsed().as_secs_f64();
    }

    /// A fan, as Glide draws polygons.
    fn draw_fan(&mut self, vertices: &[Vertex]) {
        for pair in 1..vertices.len().saturating_sub(1) {
            self.draw([vertices[0], vertices[pair], vertices[pair + 1]]);
        }
    }
}

/// How far an opaque, depth-tested triangle's corners move out, in pixels
/// of the picture drawn (not the game's).
const WIDEN: f32 = 0.5;

/// A triangle with each corner moved `by` game pixels away from its middle.
/// Extreme Assault's terrain has T-junctions: a corner of one piece lies on
/// the edge of the next, not on its corners, so a hairline gap opens along
/// that edge. At 640x480 it rarely covered a pixel's centre; at several times
/// that it showed the sky as white dotted lines between mountains (user,
/// 2026-11-16). Widened, neighbours overlap by a fraction of a pixel, and the
/// depth test settles the overlap. Only opaque, depth-tested triangles: a
/// blended one would cover its edge twice.
fn widen(mut vertices: [Vertex; 3], by: f32) -> [Vertex; 3] {
    let (x, y) = (vertices.iter().map(|v| v.x).sum::<f32>() / 3.0, vertices.iter().map(|v| v.y).sum::<f32>() / 3.0);
    for v in &mut vertices {
        let (dx, dy) = (v.x - x, v.y - y);
        let length = (dx * dx + dy * dy).sqrt();
        if length > 0.0 && length.is_finite() {
            v.x += dx / length * by;
            v.y += dy / length * by;
        }
    }
    vertices
}

/// Runs something on the Glide state, if the card is open.
fn with_state(cpu: &mut Cpu, change: impl FnOnce(&mut State)) -> u32 {
    if let Some(glide) = cpu.host.glide.as_mut() {
        change(&mut glide.state);
    }
    0
}

fn combine_arguments(cpu: &Cpu) -> Combine {
    Combine { function: cpu.arg(0), factor: cpu.arg(1), local: cpu.arg(2), other: cpu.arg(3), invert: cpu.arg(4) != 0 }
}

fn draw_triangle(cpu: &mut Cpu) -> u32 {
    let vertices = [0, 1, 2].map(|index| raster::read_vertex(cpu, cpu.arg(index)));
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw(vertices);
    }
    0
}

fn draw_vertex_list(cpu: &mut Cpu) -> u32 {
    let vertices = raster::read_list(cpu, cpu.arg(0), cpu.arg(1));
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw_fan(&vertices);
    }
    0
}

fn draw_indexed(cpu: &mut Cpu) -> u32 {
    const VERTEX_SIZE: u32 = 60;
    let (count, indices, list) = (cpu.arg(0), cpu.arg(1), cpu.arg(2));
    let vertices: Vec<Vertex> = (0..count).map(|at| raster::read_vertex(cpu, list + cpu.r32(indices + at * 4) * VERTEX_SIZE)).collect();
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.draw_fan(&vertices);
    }
    0
}

fn texture_download(cpu: &mut Cpu) -> u32 {
    let (start, info) = (cpu.arg(1), cpu.arg(3));
    if let Some(mut glide) = cpu.host.glide.take() {
        raster::download(cpu, &mut glide.state, start, info);
        cpu.host.glide = Some(glide);
    }
    0
}

/// Keeps `gu` handles apart from the card addresses the `gr` calls use.
const MANAGED_KEY: u32 = 0x8000_0000;

fn managed_allocate(cpu: &mut Cpu) -> u32 {
    let managed = Managed {
        small: cpu.arg(6),
        large: cpu.arg(7),
        aspect: cpu.arg(8),
        format: cpu.arg(4),
        clamp: [cpu.arg(9) != 0, cpu.arg(10) != 0],
        filter: cpu.arg(11) != 0 || cpu.arg(12) != 0,
    };
    let Some(glide) = cpu.host.glide.as_mut() else { return 0 };
    glide.state.managed.push(managed);
    glide.state.managed.len() as u32 - 1
}

fn managed_download(cpu: &mut Cpu) -> u32 {
    let (handle, source) = (cpu.arg(0), cpu.arg(1));
    if let Some(mut glide) = cpu.host.glide.take() {
        if let Some(texture) = glide.state.managed.get(handle as usize).copied() {
            let sizes = (texture.small, texture.large, texture.aspect, texture.format);
            raster::download_levels(cpu, &mut glide.state, MANAGED_KEY | handle, sizes, source);
        }
        cpu.host.glide = Some(glide);
    }
    0
}

fn managed_source(cpu: &mut Cpu) -> u32 {
    let handle = cpu.arg(0);
    with_state(cpu, |state| {
        if let Some(texture) = state.managed.get(handle as usize).copied() {
            state.source = Some(MANAGED_KEY | handle);
            state.clamp = texture.clamp;
            state.filter = texture.filter;
        }
    })
}

fn palette_download(cpu: &mut Cpu) -> u32 {
    const TABLE_PALETTE: u32 = 2;
    let (kind, table) = (cpu.arg(1), cpu.arg(2));
    if kind == TABLE_PALETTE {
        if let Some(mut glide) = cpu.host.glide.take() {
            raster::download_palette(cpu, &mut glide.state, table);
            cpu.host.glide = Some(glide);
        }
    }
    0
}

fn clip_window(cpu: &mut Cpu) -> u32 {
    let clip = [0, 1, 2, 3].map(|index| cpu.arg(index) as i32);
    let Some(glide) = cpu.host.glide.as_mut() else { return 0 };
    glide.state.clip = [clip[0].max(0), clip[1].max(0), clip[2].clamp(0, WIDTH as i32), clip[3].clamp(0, HEIGHT as i32)];
    0
}

/// The older one-argument way of choosing a colour combine.
fn color_combine_preset(cpu: &mut Cpu) -> u32 {
    let (function, factor, local, other) = match cpu.arg(0) {
        0 => (0, 0, 0, 0),    // Zero.
        1 => (1, 0, 1, 0),    // The constant colour.
        2 => (1, 0, 0, 0),    // The iterated colour.
        4 => (3, 8, 0, 1),    // The texture.
        5 => (3, 1, 1, 1),    // Texture times constant.
        6 => (3, 1, 0, 1),    // Texture times iterated.
        _ => (3, 8, 0, 1),
    };
    with_state(cpu, |state| state.color = Combine { function, factor, local, other, invert: false })
}

fn generate_fog(cpu: &mut Cpu) -> u32 {
    let (at, near, far) = (cpu.arg(0), f32::from_bits(cpu.arg(1)), f32::from_bits(cpu.arg(2)));
    let mut table = [0u8; 64];
    raster::linear_fog(&mut table, near, far);
    for (index, value) in table.iter().enumerate() {
        cpu.w8(at + index as u32, *value);
    }
    0
}

fn fog_table(cpu: &mut Cpu) -> u32 {
    let at = cpu.arg(0);
    let mut table = [0u8; 64];
    for (index, entry) in table.iter_mut().enumerate() {
        *entry = cpu.r8(at + index as u32);
    }
    with_state(cpu, |state| state.fog_table = table)
}

fn expand(pixel: u16) -> u32 {
    let (r, g, b) = ((pixel >> 11) as u32, ((pixel >> 5) & 63) as u32, (pixel & 31) as u32);
    ((r * 255 / 31) << 16) | ((g * 255 / 63) << 8) | (b * 255 / 31)
}

fn reduce(pixel: u32) -> u16 {
    let (r, g, b) = ((pixel >> 19) & 31, (pixel >> 10) & 63, (pixel >> 3) & 31);
    ((r << 11) | (g << 5) | b) as u16
}

const WRITE_ONLY: u32 = 1;
const FRONT_BUFFER: u32 = 0;

/// The game pixels around the pointer's hot spot its image can cover: left,
/// up, right and down.
const POINTER_REACH: [i32; 4] = [16, 16, 32, 48];

impl Glide {
    /// The pointer's area in buffer pixels: x, y, width, height.
    fn pointer_area(&self, pointer: (u32, u32)) -> (usize, usize, usize, usize) {
        let (scale, x, y) = (self.scale as i32, pointer.0 as i32, pointer.1 as i32);
        let left = (x - POINTER_REACH[0]).max(0) * scale;
        let top = (y - POINTER_REACH[1]).max(0) * scale;
        let right = ((x + POINTER_REACH[2]) * scale).min(self.width as i32);
        let bottom = ((y + POINTER_REACH[3]) * scale).min(self.height as i32);
        (left as usize, top as usize, (right - left).max(0) as usize, (bottom - top).max(0) as usize)
    }

    /// Keeps what's around the pointer before a 2D write in a 3D frame. The
    /// last such write draws the pointer, so this ends up holding what's under it.
    fn keep_under_pointer(&mut self, pointer: (u32, u32)) {
        let (x, y, width, height) = self.pointer_area(pointer);
        if let Some(gpu) = self.gpu.as_mut() {
            let area = [x as i32, y as i32, width as i32, height as i32];
            self.under_pointer_on_card = (width > 0 && height > 0).then_some(area);
            if width > 0 && height > 0 {
                gpu.keep_under_pointer(self.back, area);
            }
            return;
        }
        let buffer = &self.buffers[self.back];
        let mut pixels = Vec::with_capacity(width * height);
        for row in y..y + height {
            pixels.extend_from_slice(&buffer[row * self.width + x..row * self.width + x + width]);
        }
        self.under_pointer = (x, y, width, pixels);
    }

    /// Starts a frame that draws no 3D from the last 3D frame, without its
    /// pointer, as a fresh render would.
    fn start_from_scene(&mut self) {
        if self.restored {
            return;
        }
        self.restored = true;
        if let Some(gpu) = self.gpu.as_mut() {
            if !self.scene_on_card {
                gpu.keep_scene(1 - self.back, self.under_pointer_on_card);
                self.scene_on_card = true;
            }
            gpu.restore_scene(self.back);
            return;
        }
        if self.scene.is_none() {
            // The front buffer still shows the last 3D frame.
            let mut scene = self.buffers[1 - self.back].clone();
            let (x, y, width, ref pixels) = self.under_pointer;
            if width > 0 {
                for (row, line) in pixels.chunks(width).enumerate() {
                    let at = (y + row) * self.width + x;
                    scene[at..at + width].copy_from_slice(line);
                }
            }
            self.scene = Some(scene);
        }
        let back = self.back;
        if let Some(scene) = &self.scene {
            self.buffers[back].copy_from_slice(scene);
        }
    }

    /// The buffer a frame buffer lock names. Modal screens, such as Incubation's
    /// load and save window, draw on the front buffer and never swap.
    fn locked(&self, buffer: u32) -> usize {
        if buffer == FRONT_BUFFER { 1 - self.back } else { self.back }
    }
}

/// Pixels from one row of the linear frame buffer to the next. A Voodoo's
/// was always 2048 bytes, and some programs, such as Extreme Assault 1.1,
/// write rows that far apart without reading the stride the lock gives.
const LFB_STRIDE: usize = 1024;

/// The linear frame buffer in guest memory, which `allocate` aligns: rows
/// of `LFB_STRIDE` pixels, of which the first `WIDTH` show.
fn lfb_pixels(cpu: &Cpu, glide: &Glide) -> &'static mut [u16] {
    unsafe { std::slice::from_raw_parts_mut(cpu.memory.add(glide.lfb as usize) as *mut u16, LFB_STRIDE * HEIGHT) }
}

/// The frame buffer's shown pixels, `WIDTH` to a row.
fn lfb_read(cpu: &Cpu, glide: &Glide) -> Vec<u16> {
    lfb_pixels(cpu, glide).chunks(LFB_STRIDE).flat_map(|row| row[..WIDTH].iter().copied()).collect()
}

/// Puts `WIDTH`-wide rows into the frame buffer, and marks the rest of each
/// row unwritten.
fn lfb_write(cpu: &Cpu, glide: &Glide, flat: &[u16]) {
    for (row, source) in lfb_pixels(cpu, glide).chunks_mut(LFB_STRIDE).zip(flat.chunks(WIDTH)) {
        row[..WIDTH].copy_from_slice(source);
        row[WIDTH..].fill(UNWRITTEN);
    }
}

fn lfb_lock(cpu: &mut Cpu) -> u32 {
    const INFO_POINTER: u32 = 4;
    const INFO_STRIDE: u32 = 8;
    const INFO_WRITE_MODE: u32 = 12;
    const INFO_ORIGIN: u32 = 16;
    let (kind, buffer, info) = (cpu.arg(0), cpu.arg(1), cpu.arg(5));
    if matches!(cpu.trace_filter.as_deref(), Some("lfb" | "tally")) {
        let glide = cpu.host.glide.as_ref();
        println!(
            "lfb lock kind {kind} buffer {buffer} mode {} origin {} pipeline {} from {:#x} frame {} 3d {} queued {}",
            cpu.arg(2),
            cpu.arg(3),
            cpu.arg(4),
            cpu.r32(cpu.esp),
            glide.map_or(0, |g| g.frames),
            glide.is_some_and(|g| g.drew_3d),
            glide.is_some_and(|g| !g.batch.is_empty())
        );
    }
    const AUX_BUFFER: u32 = 2;
    if kind & 1 != WRITE_ONLY && buffer == AUX_BUFFER {
        // The depth buffer, read from its own memory (see `Glide::aux_lfb`).
        // It was filled with the farthest depth (0xffff) when Glide opened.
        let Some(aux) = cpu.host.glide.as_ref().map(|glide| glide.aux_lfb) else { return 0 };
        cpu.w32(info + INFO_POINTER, aux);
        cpu.w32(info + INFO_STRIDE, LFB_STRIDE as u32 * 2);
        cpu.w32(info + INFO_WRITE_MODE, 0);
        cpu.w32(info + INFO_ORIGIN, 0);
        return 1;
    }
    let mut glide = cpu.host.glide.take().expect("Glide isn't open");
    glide.flush();
    let started = std::time::Instant::now();
    if kind & 1 == WRITE_ONLY && buffer != FRONT_BUFFER && !glide.drew_3d {
        glide.start_from_scene();
    }
    let locked = glide.locked(buffer);
    let mut flat = vec![UNWRITTEN; WIDTH * HEIGHT];
    let pixels = &mut flat[..];
    if kind & 1 == WRITE_ONLY {
        // Left unwritten: only what the program writes is taken in.
    } else if let Some(gpu) = glide.gpu.as_mut() {
        gpu.read_flat(locked, pixels);
    } else {
        let source = &glide.buffers[locked];
        for (y, row) in pixels.chunks_mut(WIDTH).enumerate() {
            let line = &source[y * glide.scale * glide.width..];
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = reduce(line[x * glide.scale]);
            }
        }
    }
    lfb_write(cpu, &glide, &flat);
    if kind & 1 == WRITE_ONLY {
        glide.held = Some(buffer);
        // The pixel pipeline, the lock's fifth argument: see `Glide::pipeline`.
        glide.pipeline = (cpu.arg(4) != 0).then(|| Pipeline::new((glide.state.blend, glide.state.color.invert)));
    }
    cpu.w32(info + INFO_POINTER, glide.lfb);
    cpu.w32(info + INFO_STRIDE, LFB_STRIDE as u32 * 2);
    cpu.w32(info + INFO_WRITE_MODE, 0);
    cpu.w32(info + INFO_ORIGIN, 0);
    glide.spent[1] += started.elapsed().as_secs_f64();
    glide.spent[2] += 1.0;
    cpu.host.glide = Some(glide);
    1
}

fn lfb_unlock(cpu: &mut Cpu) -> u32 {
    let (kind, target) = (cpu.arg(0), cpu.arg(1));
    if kind & 1 != WRITE_ONLY {
        return 1;
    }
    // Still held while committing, for `record_writes`.
    commit_lfb(cpu, target, false);
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.held = None;
        glide.pipeline = None;
    }
    1
}

/// Notes which pixels the game wrote since the last change of mode, under
/// the mode in force (see `Pipeline`). Rows the game didn't touch are passed
/// over whole. When a pixel is written again under another mode before its
/// earlier write was taken in, the waiting writes are drawn first, so the two
/// blend in the order the game made them: the cockpit frame's shadow is
/// multiplied over the text and box beneath it, not put in their place.
fn record_writes(cpu: &Cpu, glide: &mut Glide) {
    let memory = lfb_pixels(cpu, glide);
    let Some(pipeline) = glide.pipeline.as_ref() else { return };
    let (changed, overlaps) = pipeline.changes(memory);
    if overlaps {
        if let Some(target) = glide.held {
            draw_pending(cpu, glide, target);
        }
    }
    glide.pipeline.as_mut().unwrap().note(&changed);
}

impl Pipeline {
    /// The pixels of `memory` (rows `LFB_STRIDE` apart) that differ from
    /// `before`, and whether any of them lands on a write still waiting under
    /// another mode, which must be taken in first.
    fn changes(&self, memory: &[u16]) -> (Vec<(usize, u16)>, bool) {
        let mut changed = Vec::new();
        let mut overlaps = false;
        for y in 0..HEIGHT {
            let row = &memory[y * LFB_STRIDE..][..WIDTH];
            let base = y * WIDTH;
            if row == &self.before[base..base + WIDTH] {
                continue;
            }
            for (x, &pixel) in row.iter().enumerate() {
                let at = base + x;
                if pixel != self.before[at] {
                    overlaps |= self.values[at] != UNWRITTEN && self.mode_of[at] != self.current;
                    changed.push((at, pixel));
                }
            }
        }
        (changed, overlaps)
    }

    /// Notes these writes under the mode in force.
    fn note(&mut self, changed: &[(usize, u16)]) {
        for &(at, pixel) in changed {
            self.before[at] = pixel;
            self.values[at] = pixel;
            self.mode_of[at] = self.current;
        }
        if let (Some(&(first, _)), Some(&(last, _))) = (changed.first(), changed.last()) {
            self.rows = (self.rows.0.min(first / WIDTH), self.rows.1.max(last / WIDTH + 1));
        }
    }
}

/// Draws the pipeline writes not yet taken in, one pass for each mode, and
/// leaves nothing waiting.
fn draw_pending(cpu: &Cpu, glide: &mut Glide, target: u32) {
    let locked = glide.locked(target);
    let Some(mut pipeline) = glide.pipeline.take() else { return };
    let (first, end) = pipeline.rows;
    let span = first * WIDTH..end.max(first) * WIDTH;
    for (index, &(blend, invert)) in pipeline.modes.iter().enumerate() {
        let mut written = 0u64;
        let mut flat = vec![UNWRITTEN; WIDTH * HEIGHT];
        for at in span.clone() {
            let value = pipeline.values[at];
            if value != UNWRITTEN && pipeline.mode_of[at] as usize == index {
                flat[at] = through_combine(value, invert);
                written += 1;
            }
        }
        if written == 0 {
            continue;
        }
        if blend != [4, 0] {
            glide.blended.0 += written;
        }
        draw_flat(cpu, glide, target, locked, &flat, blend);
    }
    pipeline.taken_in();
    glide.pipeline = Some(pipeline);
}

/// Before the blending or the colour combine changes during a pixel-pipeline
/// write lock: when the mode changes with it, notes what was written under
/// the mode until now, and makes `next` the mode in force.
fn pipeline_mode(cpu: &mut Cpu, next: impl FnOnce(&State) -> Mode) {
    let Some(mut glide) = cpu.host.glide.take() else { return };
    if glide.held.is_some() && glide.pipeline.is_some() {
        let next = next(&glide.state);
        let current = glide.pipeline.as_ref().map(|pipeline| pipeline.modes[pipeline.current as usize]);
        if current != Some(next) {
            let started = std::time::Instant::now();
            record_writes(cpu, &mut glide);
            glide.spent[3] += started.elapsed().as_secs_f64();
            glide.spent[4] += 1.0;
            let pipeline = glide.pipeline.as_mut().unwrap();
            let index = pipeline.modes.iter().position(|mode| *mode == next).unwrap_or_else(|| {
                pipeline.modes.push(next);
                pipeline.modes.len() - 1
            });
            pipeline.current = index.min(u8::MAX as usize) as u8;
        }
    }
    cpu.host.glide = Some(glide);
}

/// A pixel written through the pixel pipeline, as the colour combine makes it
/// before blending: inverted when the combine inverts (Extreme Assault's
/// shadows and the dark backgrounds it multiplies the 3D with). The written
/// colour stays the colour otherwise, whatever the combine's other input: with
/// the constant colour in its place, the HUD came out as flat panels.
fn through_combine(pixel: u16, invert: bool) -> u16 {
    if pixel == UNWRITTEN {
        return pixel;
    }
    let colour = if invert { !pixel } else { pixel };
    // The marker means "not written": move off it by the least step.
    if colour == UNWRITTEN { colour ^ 1 } else { colour }
}

/// One colour component blended as a Voodoo does, from 0 to 1: Glide's
/// source and destination factors, with no alpha in the frame buffer.
fn blend_component(source: f32, destination: f32, [from_source, from_destination]: [u32; 2]) -> f32 {
    let factor = |factor: u32, for_destination: bool| match factor {
        0 | 5 | 7 => 0.0,
        2 => if for_destination { source } else { destination },
        6 => if for_destination { 1.0 - source } else { 1.0 - destination },
        // As a destination factor, the source colour before fog; as a source
        // factor, alpha saturation, which is 0 without alpha.
        15 => if for_destination { source } else { 0.0 },
        _ => 1.0,
    };
    (source * factor(from_source, false) + destination * factor(from_destination, true)).clamp(0.0, 1.0)
}

/// A 0xRRGGBB pixel blended over another.
fn blend_pixel(source: u32, destination: u32, blend: [u32; 2]) -> u32 {
    [16, 8, 0].iter().fold(0, |out, shift| {
        let (s, d) = (((source >> shift) & 255) as f32 / 255.0, ((destination >> shift) & 255) as f32 / 255.0);
        out | (((blend_component(s, d, blend) * 255.0).round() as u32) << shift)
    })
}

/// Puts what the game wrote to the linear frame buffer into `target`. With
/// `whole`, all of it, copied, as at a swap made with the lock still held;
/// otherwise, for a pixel-pipeline lock, what was written since it was last
/// taken in, one pass for each blending mode (see `Pipeline`).
fn commit_lfb(cpu: &mut Cpu, target: u32, whole: bool) {
    let mut glide = cpu.host.glide.take().expect("Glide isn't open");
    glide.flush();
    let started = std::time::Instant::now();
    let locked = glide.locked(target);
    if target == FRONT_BUFFER {
        glide.updates += 1;
    } else if glide.drew_3d {
        glide.keep_under_pointer(cpu.host.pointer);
    }
    if whole || glide.pipeline.is_none() {
        let all = lfb_read(cpu, &glide);
        if let Some(pipeline) = glide.pipeline.as_mut() {
            pipeline.before.copy_from_slice(&all);
            pipeline.taken_in();
        }
        draw_flat(cpu, &mut glide, target, locked, &all, [4, 0]);
    } else {
        record_writes(cpu, &mut glide);
        draw_pending(cpu, &mut glide, target);
    }
    glide.spent[1] += started.elapsed().as_secs_f64();
    cpu.host.glide = Some(glide);
}

/// Draws 640x480 pixels of 2D, where the marker means nothing was written,
/// into a buffer, copied or blended.
fn draw_flat(cpu: &Cpu, glide: &mut Glide, target: u32, locked: usize, pixels: &[u16], blend: [u32; 2]) {
    let (scale, width) = (glide.scale, glide.width);
    if blend == [4, 0] {
        glide.copied.0 += pixels.iter().filter(|&&pixel| pixel != UNWRITTEN).count() as u64;
    }
    if cpu.trace_filter.as_deref() == Some("lfb") {
        let written: Vec<usize> = (0..pixels.len()).filter(|&at| pixels[at] != UNWRITTEN).collect();
        let (xs, ys) = (written.iter().map(|at| at % WIDTH), written.iter().map(|at| at / WIDTH));
        let (left, right) = (xs.clone().min().unwrap_or(0), xs.max().unwrap_or(0));
        let (top, bottom) = (ys.clone().min().unwrap_or(0), ys.max().unwrap_or(0));
        println!("lfb unlock buffer {target}: {} pixels written, x {left}-{right}, y {top}-{bottom}, blend {blend:?}", written.len());
    }
    if let Some(gpu) = glide.gpu.as_mut() {
        gpu.write_flat(locked, pixels, blend);
        return;
    }
    // Each game row covers `scale` rows of the buffer. Runs of written pixels
    // are widened once, then copied to each of those rows, and the rows are
    // shared out among the threads, since this runs at the full resolution.
    let rows_per_job = 8;
    let jobs = Mutex::new(glide.buffers[locked].chunks_mut(rows_per_job * scale * width).zip(pixels.chunks(rows_per_job * WIDTH)).collect::<Vec<_>>());
    let work = || {
        let mut wide = Vec::with_capacity(WIDTH * scale);
        loop {
            let job = jobs.lock().unwrap().pop();
            let Some((buffer, pixels)) = job else { break };
            for (lines, row) in buffer.chunks_mut(scale * width).zip(pixels.chunks(WIDTH)) {
                let mut x = 0;
                while x < row.len() {
                    if row[x] == UNWRITTEN {
                        x += 1;
                        continue;
                    }
                    let start = x;
                    while x < row.len() && row[x] != UNWRITTEN {
                        x += 1;
                    }
                    wide.clear();
                    for &value in &row[start..x] {
                        let colour = expand(value);
                        wide.extend(std::iter::repeat_n(colour, scale));
                    }
                    for line in lines.chunks_mut(width) {
                        let target = &mut line[start * scale..x * scale];
                        if blend == [4, 0] {
                            target.copy_from_slice(&wide);
                        } else {
                            for (pixel, &colour) in target.iter_mut().zip(&wide) {
                                *pixel = blend_pixel(colour, *pixel, blend);
                            }
                        }
                    }
                }
            }
        }
    };
    glide.pool.run(&work);
}

fn buffer_clear(cpu: &mut Cpu) -> u32 {
    const FAR: u32 = 0xffff;
    let (color, depth) = (cpu.arg(0), cpu.arg(2) & 0xffff);
    let trace_modes = cpu.trace_filter.as_deref() == Some("tally");
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.flush();
        if trace_modes {
            println!("frame {} clear colour {color:#x} ({}) depth {depth:#x} ({})", glide.frames, glide.state.color_write, glide.state.depth_write);
        }
        // Only clearing the colour starts a fresh picture. Incubation's load
        // and save window clears just the depth each frame.
        glide.drew_3d |= glide.state.color_write;
        let (back, depth_write, color_write) = (glide.back, glide.state.depth_write, glide.state.color_write);
        if let Some(gpu) = glide.gpu.as_mut() {
            // The card's depth buffer holds 1/w, so the farthest is 0.
            let far = if depth == FAR { 0.0 } else { 1.0 / depth as f32 };
            if depth_write || color_write {
                gpu.clear(back, color_write.then_some(color & 0xff_ffff), depth_write.then_some(far));
            }
            return 0;
        }
        if glide.state.depth_write {
            glide.depth.fill(if depth == FAR { f32::INFINITY } else { depth as f32 });
        }
        if glide.state.color_write {
            let back = glide.back;
            glide.buffers[back].fill(color & 0xff_ffff);
        }
    }
    0
}

fn buffer_swap(cpu: &mut Cpu) -> u32 {
    // The swap interval: how many of the Voodoo's refreshes, at 60 a second,
    // to wait since the last swap. Extreme Assault swaps its briefing static
    // (`DISTORT`) with 1, and its normal frames with 0; without the wait the
    // static went by in a few hundredths of a second on a fast display.
    const VOODOO_REFRESH: f64 = 1.0 / 60.0;
    let interval = cpu.arg(0).min(4);
    if cpu.trace_filter.as_deref() == Some("lfb") && interval > 0 {
        // Extreme Assault 1.1's swap wrapper (0x20a44e9) keeps its caller 36 bytes up.
        let since = cpu.host.glide.as_ref().map_or(0.0, |glide| glide.last_swap.elapsed().as_secs_f64() * 1000.0);
        println!("swap interval {interval} from {:#x}, its caller {:#x}, {since:.1} ms since the last", cpu.r32(cpu.esp), cpu.r32(cpu.esp + 36));
    }
    if interval > 0 {
        if let Some(glide) = cpu.host.glide.as_ref() {
            let due = glide.last_swap + std::time::Duration::from_secs_f64(VOODOO_REFRESH * interval as f64);
            let now = std::time::Instant::now();
            if due > now {
                std::thread::sleep(due - now);
            }
        }
    }
    // A lock still held: its writes go to the picture being swapped in.
    if let Some(target) = cpu.host.glide.as_ref().and_then(|glide| glide.held) {
        commit_lfb(cpu, target, true);
    }
    let trace_modes = cpu.trace_filter.as_deref() == Some("tally");
    if let Some(glide) = cpu.host.glide.as_mut() {
        glide.flush();
        if trace_modes {
            for (mode, count) in glide.mode_tally.replace(Default::default()).unwrap_or_default() {
                println!("frame {} {count} triangles: {mode}", glide.frames);
            }
        }
        glide.back = 1 - glide.back;
        if glide.drew_3d {
            glide.scene = None;
            glide.scene_on_card = false;
        }
        if let Some(gpu) = glide.gpu.as_mut() {
            gpu.collect();
        }
        glide.drew_3d = false;
        glide.restored = false;
        glide.blended = (0, glide.blended.0);
        glide.copied = (0, glide.copied.0);
        glide.frames += 1;
        glide.updates += 1;
        let now = std::time::Instant::now();
        let frame = (now - std::mem::replace(&mut glide.last_swap, now)).as_secs_f64();
        glide.longest_frame = glide.longest_frame.max(frame);
    }
    // Paced as DirectDraw's flips are; this also feeds the frame counter.
    // (Leaving the wait to the present alone, tried on 2026-11-19, changed
    // nothing on the player's machine and left the counter stale, at 3 fps:
    // the 30 fps were the game's own limiter, see `mods/glide.rs`.)
    super::ddraw::wait_for_refresh(cpu);
    super::video::pump(cpu);
    0
}

fn window_open(cpu: &mut Cpu) -> u32 {
    let glide = Glide::new(cpu);
    if let Some(video) = cpu.host.video.as_mut().filter(|video| video.gl().is_none()) {
        video.use_glide_size(glide.width, glide.height);
    }
    cpu.host.glide = Some(glide);
    1
}

/// How many pixels of 2D the last whole frame blended through the pixel
/// pipeline rather than copied, for the script's `blended:` check.
pub fn blended_last_frame(cpu: &Cpu) -> u64 {
    cpu.host.glide.as_ref().map_or(0, |glide| glide.blended.1)
}

/// How many pixels of 2D the last whole frame copied over the 3D, for the
/// script's `copied:` check.
pub fn copied_last_frame(cpu: &Cpu) -> u64 {
    cpu.host.glide.as_ref().map_or(0, |glide| glide.copied.1)
}

/// The picture most recently swapped to the front, as 0xRRGGBB values, when
/// the processor draws.
pub fn front(cpu: &Cpu) -> Option<(usize, usize, &[u32])> {
    let glide = cpu.host.glide.as_ref().filter(|glide| glide.gpu.is_none())?;
    Some((glide.width, glide.height, &glide.buffers[1 - glide.back]))
}

/// The same picture as a texture, when the graphics card draws.
pub fn front_texture(cpu: &Cpu) -> Option<u32> {
    let glide = cpu.host.glide.as_ref()?;
    Some(glide.gpu.as_ref()?.texture(1 - glide.back))
}

/// The same picture read back from the graphics card, for a screenshot.
pub fn front_from_card(cpu: &Cpu) -> Option<(usize, usize, Vec<u32>)> {
    let glide = cpu.host.glide.as_ref()?;
    Some(glide.gpu.as_ref()?.read(1 - glide.back))
}

macro_rules! stubs {
    ($($bytes:literal)*) => {
        fn stub(bytes: u32) -> Option<GuestFn> {
            match bytes {
                $($bytes => Some((|cpu: &mut Cpu| cpu.finish(0, $bytes / 4)) as GuestFn),)*
                _ => None,
            }
        }
    };
}
stubs!(0 4 8 12 16 20 24 28 32 36 40 44 48 52 56 60 64);

fn query_hardware(cpu: &mut Cpu) -> u32 {
    const VOODOO: u32 = 0;
    let at = cpu.arg(0);
    cpu.w32(at, 1); // num_sst
    cpu.w32(at + 4, VOODOO);
    cpu.w32(at + 8, 4); // fbRam
    cpu.w32(at + 12, 2); // fbiRev
    cpu.w32(at + 16, 1); // nTexelfx
    cpu.w32(at + 20, 0); // sliDetect
    cpu.w32(at + 24, 1); // tmuRev
    cpu.w32(at + 28, 4); // tmuRam
    1
}

/// The functions `find` knows by name, as the Windows DLL spells them.
const KNOWN: &[&str] = &[
    "_grSstQueryHardware@4", "_grSstQueryBoards@4", "_grSstWinOpen@28", "_grSstScreenWidth@0", "_grSstScreenHeight@0",
    "_grLfbLock@24", "_grLfbUnlock@8", "_grBufferClear@12", "_grBufferSwap@4", "_guColorCombineFunction@4",
    "_guFogGenerateLinear@12", "_guTexAllocateMemory@60", "_guTexDownloadMipMap@12", "_guTexSource@4", "_guTexMemReset@0",
    "_guTexMemQueryAvail@4", "_grTexMinAddress@4", "_grTexMaxAddress@4", "_grTexTextureMemRequired@8", "_grTexCalcMemRequired@16",
    "_grTexDownloadMipMap@16", "_grTexDownloadTable@12", "_grTexSource@16", "_grTexFilterMode@12", "_grTexClampMode@12",
    "_grColorCombine@20", "_grAlphaCombine@20", "_grAlphaBlendFunction@16", "_grAlphaTestFunction@4", "_grAlphaTestReferenceValue@4",
    "_grChromakeyMode@4", "_grChromakeyValue@4", "_grConstantColorValue@4", "_grCullMode@4", "_grDepthBufferMode@4",
    "_grDepthBufferFunction@4", "_grDepthMask@4", "_grColorMask@8", "_grFogMode@4", "_grFogColorValue@4", "_grFogTable@4",
    "_grClipWindow@16", "_grDrawTriangle@12", "_grAADrawTriangle@24", "_grDrawPolygonVertexList@8",
    "_grDrawPlanarPolygonVertexList@8", "_grAADrawPolygonVertexList@8", "_grDrawPolygon@12", "_grDrawPlanarPolygon@12",
    "_grAADrawPolygon@12",
];

/// A Glide function by a name in any case. DOS programs name them in upper
/// case, such as `_GRDRAWTRIANGLE@12`, as the DOS overlay `GLIDE2X.OVL` does.
pub fn find_ignoring_case(name: &str) -> Option<GuestFn> {
    let spelled = KNOWN.iter().find(|known| known.eq_ignore_ascii_case(name)).copied().unwrap_or(name);
    find(spelled)
}

pub fn find(name: &str) -> Option<GuestFn> {
    let known = super::table! { name;
        "_grSstQueryHardware@4" 1 |cpu| { query_hardware(cpu) }
        "_grSstQueryBoards@4" 1 |cpu| { let at = cpu.arg(0); cpu.w32(at, 1); 1 }
        "_grSstWinOpen@28" 7 |cpu| { window_open(cpu) }
        "_grSstScreenWidth@0" 0 |_cpu| { WIDTH as u32 }
        "_grSstScreenHeight@0" 0 |_cpu| { HEIGHT as u32 }
        "_grLfbLock@24" 6 |cpu| { lfb_lock(cpu) }
        "_grLfbUnlock@8" 2 |cpu| { lfb_unlock(cpu) }
        "_grBufferClear@12" 3 |cpu| { buffer_clear(cpu) }
        "_grBufferSwap@4" 1 |cpu| { buffer_swap(cpu) }
        "_guColorCombineFunction@4" 1 |cpu| { color_combine_preset(cpu) }
        "_guFogGenerateLinear@12" 3 |cpu| { generate_fog(cpu) }
        "_guTexAllocateMemory@60" 15 |cpu| { managed_allocate(cpu) }
        "_guTexDownloadMipMap@12" 3 |cpu| { managed_download(cpu) }
        "_guTexSource@4" 1 |cpu| { managed_source(cpu) }
        "_guTexMemReset@0" 0 |cpu| { with_state(cpu, |state| state.managed.clear()) }
        "_guTexMemQueryAvail@4" 1 |_cpu| { TEXTURE_MEMORY }
        "_grTexMinAddress@4" 1 |_cpu| { 0 }
        "_grTexMaxAddress@4" 1 |_cpu| { TEXTURE_MEMORY }
        "_grTexTextureMemRequired@8" 2 |cpu| {
            let info = cpu.arg(1);
            raster::memory_required(cpu.r32(info), cpu.r32(info + 4), cpu.r32(info + 8), cpu.r32(info + 12))
        }
        "_grTexCalcMemRequired@16" 4 |cpu| { raster::memory_required(cpu.arg(0), cpu.arg(1), cpu.arg(2), cpu.arg(3)) }
        "_grTexDownloadMipMap@16" 4 |cpu| { texture_download(cpu) }
        "_grTexDownloadTable@12" 3 |cpu| { palette_download(cpu) }
        "_grTexSource@16" 4 |cpu| { let start = cpu.arg(1); with_state(cpu, |state| state.source = Some(start)) }
        "_grTexFilterMode@12" 3 |cpu| { let filter = cpu.arg(1) != 0 || cpu.arg(2) != 0; with_state(cpu, |state| state.filter = filter) }
        "_grTexClampMode@12" 3 |cpu| { let clamp = [cpu.arg(1) != 0, cpu.arg(2) != 0]; with_state(cpu, |state| state.clamp = clamp) }
        "_grColorCombine@20" 5 |cpu| { let unit = combine_arguments(cpu); pipeline_mode(cpu, |state| (state.blend, unit.invert)); with_state(cpu, |state| state.color = unit) }
        "_grAlphaCombine@20" 5 |cpu| { let unit = combine_arguments(cpu); with_state(cpu, |state| state.alpha = unit) }
        "_grAlphaBlendFunction@16" 4 |cpu| { let blend = [cpu.arg(0), cpu.arg(1)]; pipeline_mode(cpu, |state| (blend, state.color.invert)); with_state(cpu, |state| state.blend = blend) }
        "_grAlphaTestFunction@4" 1 |cpu| { let function = cpu.arg(0); with_state(cpu, |state| state.alpha_test = function) }
        "_grAlphaTestReferenceValue@4" 1 |cpu| { let value = (cpu.arg(0) & 255) as f32; with_state(cpu, |state| state.alpha_reference = value) }
        "_grChromakeyMode@4" 1 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.chroma = on) }
        "_grChromakeyValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.chroma_value = value) }
        "_grConstantColorValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.constant = value) }
        "_grCullMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.cull = mode) }
        "_grDepthBufferMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.depth_mode = mode) }
        "_grDepthBufferFunction@4" 1 |cpu| { let function = cpu.arg(0); with_state(cpu, |state| state.depth_function = function) }
        "_grDepthMask@4" 1 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.depth_write = on) }
        "_grColorMask@8" 2 |cpu| { let on = cpu.arg(0) != 0; with_state(cpu, |state| state.color_write = on) }
        "_grFogMode@4" 1 |cpu| { let mode = cpu.arg(0); with_state(cpu, |state| state.fog_mode = mode) }
        "_grFogColorValue@4" 1 |cpu| { let value = cpu.arg(0); with_state(cpu, |state| state.fog_color = value) }
        "_grFogTable@4" 1 |cpu| { fog_table(cpu) }
        "_grClipWindow@16" 4 |cpu| { clip_window(cpu) }
        "_grDrawTriangle@12" 3 |cpu| { draw_triangle(cpu) }
        "_grAADrawTriangle@24" 6 |cpu| { draw_triangle(cpu) }
        "_grDrawPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grDrawPlanarPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grAADrawPolygonVertexList@8" 2 |cpu| { draw_vertex_list(cpu) }
        "_grDrawPolygon@12" 3 |cpu| { draw_indexed(cpu) }
        "_grDrawPlanarPolygon@12" 3 |cpu| { draw_indexed(cpu) }
        "_grAADrawPolygon@12" 3 |cpu| { draw_indexed(cpu) }
    };
    known.or_else(|| {
        let (_, bytes) = name.rsplit_once('@')?;
        stub(bytes.parse().ok()?)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mip_level_as_a_voodoo() {
        use super::super::glide_raster::voodoo_lod;
        let at = [(0.0, 0.0), (100.0, 0.0), (0.0, 100.0)];
        // One texel a pixel: the largest level.
        assert_eq!(voodoo_lod(at, [(0.0, 0.0), (100.0, 0.0), (0.0, 100.0)]), 0);
        // Four texels a pixel both ways: level 2.
        assert_eq!(voodoo_lod(at, [(0.0, 0.0), (400.0, 0.0), (0.0, 400.0)]), 2);
        // Slanted ground, squeezed 8 times one way only: the steeper way
        // counts, as on the card, so it blurs (level 3, not the area's 1.5).
        assert_eq!(voodoo_lod(at, [(0.0, 0.0), (100.0, 0.0), (0.0, 800.0)]), 3);
    }

    #[test]
    fn pixel_pipeline_blending() {
        // Copy, add, and multiply by the source (Extreme Assault's shadows).
        assert_eq!(blend_pixel(0x804020, 0x102030, [4, 0]), 0x804020);
        assert_eq!(blend_pixel(0x804020, 0x102030, [4, 4]), 0x906050);
        assert_eq!(blend_pixel(0x808080, 0xff4000, [0, 15]), 0x802000);
        // The inverted source of a shadow darkens by its own value.
        let mut state = State::default();
        state.color.invert = true;
        assert_eq!(through_combine(0x0000, state.color.invert), 0xffff);
        assert_eq!(through_combine(UNWRITTEN, state.color.invert), UNWRITTEN);
    }

    /// The cockpit frame's shadow, multiplied over text written just before
    /// it, must blend over the text, not replace it (the briefing's tip box
    /// lost its left part under the shadow).
    #[test]
    fn pipeline_overlapping_writes_are_taken_in_first() {
        let mut memory = vec![UNWRITTEN; LFB_STRIDE * HEIGHT];
        let mut pipeline = Pipeline::new(([4, 4], false));
        // Text, added, at (10, 5).
        memory[5 * LFB_STRIDE + 10] = 0xffff;
        let (changed, overlaps) = pipeline.changes(&memory);
        assert_eq!((changed.len(), overlaps), (1, false));
        pipeline.note(&changed);
        // The shadow mode: a write elsewhere doesn't overlap...
        pipeline.modes.push(([0, 15], true));
        pipeline.current = 1;
        memory[6 * LFB_STRIDE + 10] = 0x1234;
        assert_eq!(pipeline.changes(&memory), (vec![(6 * WIDTH + 10, 0x1234)], false));
        // ...but one over the waiting text does.
        memory[5 * LFB_STRIDE + 10] = 0x4321;
        assert!(pipeline.changes(&memory).1);
        // Once taken in, nothing waits, so it no longer overlaps.
        pipeline.taken_in();
        assert!(!pipeline.changes(&memory).1);
    }
}
