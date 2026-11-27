//! Sets up guest memory and starts the game.
//!
//! The game's data sections are read from the player's own files at run time,
//! at the same addresses the recompiler used.

use crate::cpu::{Cpu, GuestFn, Host, IMPORT_BASE};
use crate::{host, pe};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const STACK_TOP: u32 = 0x7000_0000;
const STACK_SIZE: u32 = 0x0040_0000;
const THREAD_BLOCK: u32 = 0x7ffd_0000;
/// Rust frames stand in for the guest's calls, so the host stack has to be deep.
const HOST_STACK: usize = 512 * 1024 * 1024;

/// DLLs Windows would start before the program, dependencies first. The others
/// wait for LoadLibrary.
const STARTED_FIRST: &[&str] = &["msvcrt.dll", "msvcirt.dll", "newtoolsr.dll"];

pub struct Module {
    pub file: &'static str,
    pub base: u32,
    pub sha256: &'static str,
    /// A program, which starts a program of its own with the DLLs after it.
    /// A port can carry several, such as a game's DOS and Windows versions,
    /// and starts the one the game folder holds.
    pub program: bool,
    /// The version players know a program by, from `recompVersionNames`, such
    /// as `patched to 1.23`. The launcher names the version it finds with it.
    pub name: &'static str,
}

/// The name of the version in a folder or disc image, when its program is
/// one the port carries and has a name for.
pub(crate) fn version_in(game: &Game, place: &str) -> Option<&'static str> {
    let path = std::path::Path::new(place);
    let program = if host::disc::is_image(path) {
        programs_on_image(game, path).first().copied()?
    } else {
        let program = program_in(game, place);
        let path = find_file(place, program[0].file)?;
        (sha256_of(&path) == program[0].sha256).then_some(program)?
    };
    Some(program[0].name).filter(|name| !name.is_empty())
}

/// The port's programs that a disc image holds, by their contents.
fn programs_on_image(game: &Game, image: &std::path::Path) -> Vec<&'static [Module]> {
    let Ok(mut disc) = host::disc::Disc::open(image) else { return Vec::new() };
    let files = disc.files();
    let mut found = programs(game);
    found.retain(|program| {
        let name = without_tag(program[0].file);
        let places: Vec<String> = std::iter::once(name.to_string()).chain(install_folders().iter().map(|folder| format!("{folder}/{name}"))).collect();
        let Some(entry) = places.iter().find_map(|place| files.iter().find(|entry| entry.path.eq_ignore_ascii_case(place))) else { return false };
        disc.read(entry).is_ok_and(|data| {
            let found: String = Sha256::digest(&data).iter().map(|byte| format!("{byte:02x}")).collect();
            log_hash(&format!("{} : {}", image.display(), entry.path), data.len(), &found);
            found == program[0].sha256
        })
    });
    found
}

/// Whether the folder has each of these programs, the same builds.
fn holds(folder: &std::path::Path, wanted: &[&[Module]]) -> bool {
    let folder = folder.to_string_lossy();
    !wanted.is_empty() && wanted.iter().all(|program| program.iter().all(|module| find_file(&folder, module.file).is_some_and(|path| sha256_of(&path) == module.sha256)))
}

/// The programs a port carries: each program's module, then its DLLs.
fn programs(game: &Game) -> Vec<&'static [Module]> {
    let modules = game.modules;
    let starts: Vec<usize> = (0..modules.len()).filter(|&index| index == 0 || modules[index].program).collect();
    starts.iter().enumerate().map(|(n, &start)| &modules[start..starts.get(n + 1).copied().unwrap_or(modules.len())]).collect()
}

/// The program whose file is in the folder, the exact build first, since two
/// versions of one program can have the same name; or else the first. A
/// program of the chosen renderer comes before the others (see
/// `choose_renderer`).
pub(crate) fn program_in(game: &Game, folder: &str) -> &'static [Module] {
    let all = programs(game);
    let preferred = *PREFERRED_PROGRAM.lock().unwrap();
    let chosen = |program: &&[Module]| preferred.is_some_and(|file| same_program(program[0].file, file));
    let matches = |program: &&[Module]| find_file(folder, program[0].file).is_some_and(|path| sha256_of(&path) == program[0].sha256);
    let present = |program: &&[Module]| find_file(folder, program[0].file).is_some();
    let first = |test: &dyn Fn(&&[Module]) -> bool| all.iter().copied().find(|program| test(program));
    first(&|p| chosen(p) && matches(p)).or_else(|| first(&matches)).or_else(|| first(&|p| chosen(p) && present(p))).or_else(|| first(&present)).unwrap_or(all[0])
}

/// The program file a renderer stands for, when the game's renderers are
/// separate programs, as Extreme Assault's `XA_3DFX.EXE` (Glide) and `XA.EXE`
/// (software) are: `recompRenderers` then names the file in place of a switch.
static PREFERRED_PROGRAM: std::sync::Mutex<Option<&'static str>> = std::sync::Mutex::new(None);

/// Whether a module's file, such as `XA.EXE@1.1`, is the program `file` names.
fn same_program(module: &str, file: &str) -> bool {
    let name = without_tag(module);
    name.rsplit('/').next().unwrap_or(name).eq_ignore_ascii_case(file)
}

/// The program a renderer's switch names, if it names one of the port's programs.
fn renderer_program(game: &Game, index: usize) -> Option<&'static str> {
    let (switch, _) = *game.renderers.get(index)?;
    programs(game).iter().any(|program| same_program(program[0].file, switch)).then_some(switch)
}

/// Makes the programs of the chosen renderer the ones started, when there's one in the folder.
pub(crate) fn choose_renderer(game: &Game, index: usize) {
    *PREFERRED_PROGRAM.lock().unwrap() = renderer_program(game, index);
}

/// Whether the folder can run the renderer: always, unless the renderer is a
/// program of its own that the folder lacks, such as Glide on the German disc,
/// which has only the software `XA.EXE`. A disc image has its own programs, or
/// those of its unpacked copy, which a patch may have added to.
pub(crate) fn renderer_available(game: &Game, folder: &str, index: usize) -> bool {
    let Some(file) = renderer_program(game, index) else { return index < game.renderers.len() };
    let path = std::path::Path::new(folder);
    let unpacked;
    let mut folder = folder;
    if host::disc::is_image(path) {
        match unpacked_copy(game, path) {
            Some(copy) => {
                unpacked = copy.display().to_string();
                folder = &unpacked;
            }
            None => return programs_on_image(game, path).iter().any(|program| same_program(program[0].file, file)),
        }
    }
    programs(game).iter().any(|program| same_program(program[0].file, file) && find_file(folder, program[0].file).is_some_and(|path| sha256_of(&path) == program[0].sha256))
}

/// The chosen renderer if the folder can run it, or else the first it can.
pub(crate) fn usable_renderer(game: &Game, folder: &str, chosen: usize) -> usize {
    if game.renderers.is_empty() || renderer_available(game, folder, chosen) {
        return chosen;
    }
    (0..game.renderers.len()).find(|&index| renderer_available(game, folder, index)).unwrap_or(chosen)
}

fn sha256_of(path: &std::path::Path) -> String {
    let file = std::fs::read(path).unwrap_or_default();
    let found: String = Sha256::digest(&file).iter().map(|byte| format!("{byte:02x}")).collect();
    log_hash(&path.display().to_string(), file.len(), &found);
    found
}

/// Writes a checked file's hash to `recomp.log`, once per file and contents, so
/// a player's log says which version they have even when it isn't a supported
/// one. The supported versions' hashes are in the README.
fn log_hash(place: &str, size: usize, found: &str) {
    static LOGGED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
    let key = format!("{place} {found}");
    let mut logged = LOGGED.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if !logged.contains(&key) {
        crate::runlog::line(&format!("checked {place}: {size} bytes, sha256 {found}"));
        logged.push(key);
    }
}

/// Folders of an unpacked disc whose files the game's installer copied to the
/// game's own folder, from `recompInstall`, such as `EXTREME/ROOT` and then
/// `EXTREME`. The game then runs on the disc's files as if installed: a file
/// that isn't in the game folder is looked for in these.
static INSTALL_FOLDERS: std::sync::OnceLock<&'static [&'static str]> = std::sync::OnceLock::new();

pub(crate) fn install_folders() -> &'static [&'static str] {
    INSTALL_FOLDERS.get().copied().unwrap_or(&[])
}

/// The game's CD as its table of contents says, from `recompCdTracks`: where
/// each track starts, in sectors from the start of the disc, track 1 first,
/// and where the disc ends. Empty when the port doesn't give it.
static CD_TRACKS: std::sync::OnceLock<&'static [u32]> = std::sync::OnceLock::new();

pub(crate) fn cd_tracks() -> &'static [u32] {
    CD_TRACKS.get().copied().unwrap_or(&[])
}

/// A module's file name without the tag that tells two versions apart, as
/// in `XA_3DFX.EXE@1.1`.
fn without_tag(name: &str) -> &str {
    name.split_once('@').map_or(name, |(path, _)| path)
}

pub struct Game {
    pub title: &'static str,
    /// The release, from the generated repo's `VERSION` file, such as `0.5-beta`.
    pub version: &'static str,
    pub needs: &'static str,
    pub command: &'static str,
    /// Command-line switches, the guest address each sets to 1, and its label.
    /// With address 0 the game reads the switch itself, from its command line.
    pub switches: &'static [(&'static str, u32, &'static str)],
    /// The renderers the launcher offers, from `recompRenderers`: the game's own switch that
    /// picks each, or the program file that is that renderer, and its label. The first is the
    /// default.
    pub renderers: &'static [(&'static str, &'static str)],
    /// The languages the launcher offers, from `recompLanguages`, the same way.
    pub languages: &'static [(&'static str, &'static str)],
    /// Mods the player can turn off in the launcher: the file in `mods/`, its
    /// label, whether it starts on, and whether its row is on the launcher's
    /// advanced page. Mods not listed are always on.
    pub mods: &'static [(&'static str, &'static str, bool, bool)],
    pub modules: &'static [Module],
    /// Each program's translated functions, by address: one table per program crate.
    pub tables: &'static [&'static [(u32, GuestFn)]],
    pub icon: &'static [u8],
    /// CD music files, from `recompMusic`: see `host::State::music`.
    pub music: &'static [(&'static str, u32, u32)],
    /// Registry strings the game expects from its installer, from
    /// `recompRegistry`: the key and name, and the text.
    pub registry: &'static [(&'static str, &'static str)],
    /// The volume label of the game's CD, from `recompCd`. With one, the game
    /// finds a CD drive D: with that label, which is the game folder.
    pub cd: &'static str,
    /// Folders of the disc that its installer copied into the game's folder,
    /// from `recompInstall`: see `install_folders`.
    pub install: &'static [&'static str],
    /// The CD's track layout, from `recompCdTracks`: see `cd_tracks`.
    pub cd_tracks: &'static [u32],
    /// Exit codes besides 0 that mean the player quit, from `recompNormalExits`.
    pub normal_exits: &'static [i32],
}

/// A program with no console would otherwise end without a word, so a crash
/// goes into `recomp.log` (and `recomp-crash.log` here, as before) and tells the player.
fn report_crashes(title: &'static str) {
    std::panic::set_hook(Box::new(move |information| {
        let text = format!("{title} stopped with an error:\n\n{information}");
        let backtrace = std::backtrace::Backtrace::force_capture();
        crate::runlog::write_panic(&format!("\n{text}\n\nbacktrace:\n{backtrace}\n"));
        let _ = std::fs::write("recomp-crash.log", &text);
        let text = match crate::runlog::path() {
            Some(path) => format!("{text}\n\nPlease send this file with your report:\n{}", path.display()),
            None => text,
        };
        if crate::runlog::automated() {
            eprintln!("{text}");
        } else {
            host::video::show_error(title, &text);
        }
    }));
}

#[cfg(test)]
mod crash_tests;

/// What the run chose, at the top of `recomp.log`.
fn log_settings(game: &Game, settings: &Settings, folder: &str, command: &str) {
    let on = |list: Vec<&str>| if list.is_empty() { "none".to_string() } else { list.join(", ") };
    crate::runlog::line(&format!("game folder: {folder}"));
    if let Some((_, label)) = game.renderers.get(settings.renderer) {
        crate::runlog::line(&format!("renderer: {label}"));
    }
    if let Some((_, label)) = game.languages.get(settings.language) {
        crate::runlog::line(&format!("language: {label}"));
    }
    let switches: Vec<&str> = game.switches.iter().zip(&settings.switches).filter(|(_, on)| **on).map(|((name, _, _), _)| *name).collect();
    crate::runlog::line(&format!("switches: {}", on(switches)));
    let off: Vec<&str> = game.mods.iter().zip(&settings.mods).filter(|(_, on)| !**on).map(|((file, _, _, _), _)| *file).collect();
    crate::runlog::line(&format!("mods off: {}", on(off)));
    crate::runlog::line(&format!("scale {}, fullscreen {}, Glide on the processor {}", settings.scale, settings.fullscreen, settings.processor));
    crate::runlog::line(&format!("command: {command}"));
    crate::runlog::line("--- the game's output ---");
}

/// Without a path, the game folder is `game` here or beside the program, or else
/// this folder or the program's own, for a build copied in with the game's files.
pub(crate) fn default_folder(game: &Game) -> String {
    let program = std::env::current_exe().ok().and_then(|path| Some(path.parent()?.to_path_buf()));
    let mut places = vec![std::path::PathBuf::from("game")];
    places.extend(program.iter().map(|folder| folder.join("game")));
    // With the XDG folders chosen, an unpacked disc is in the data folder.
    places.extend(host::launcher::files_folder().map(|folder| folder.join("game")));
    places.push(std::path::PathBuf::from("."));
    places.extend(program);
    let firsts: Vec<&str> = programs(game).iter().map(|program| program[0].file).collect();
    let found = places.iter().map(|place| place.to_string_lossy().to_string()).find(|place| firsts.iter().any(|file| find_file(place, file).is_some()));
    found.unwrap_or_else(|| "game".to_string())
}

/// Checks the files the code was recompiled from, before anything starts.
pub(crate) fn check_files(game: &Game, folder: &str) -> Result<(), String> {
    let mut missing = Vec::new();
    let mut different = Vec::new();
    for module in program_in(game, folder) {
        match find_file(folder, module.file) {
            None => missing.push(module.file),
            Some(path) => {
                if sha256_of(&path) != module.sha256 {
                    different.push(without_tag(module.file));
                }
            }
        }
    }
    let missing: Vec<&str> = missing.into_iter().map(without_tag).collect();
    if missing.is_empty() && different.is_empty() {
        return Ok(());
    }
    let absolute = std::fs::canonicalize(folder).map_or(folder.to_string(), |path| path.display().to_string());
    let mut text = format!("{} can't start, because of the files in this folder:\n{absolute}\n", game.title);
    if !missing.is_empty() {
        text.push_str(&format!("\nMissing: {}\n", missing.join(", ")));
    }
    if !different.is_empty() {
        text.push_str(&format!("\nFrom a different version: {}\n", different.join(", ")));
    }
    text.push_str(&format!(
        "\n{} needs {}.\n\nPut this program in the game's folder, or put the game's files in a folder named game beside it, or start it with the folder as its argument.",
        game.title, game.needs
    ));
    Err(text)
}

/// Checks a disc image as `check_files` checks a folder, with a reason short
/// enough for the launcher.
pub(crate) fn check_image(game: &Game, image: &std::path::Path) -> Result<(), String> {
    let mut disc = host::disc::Disc::open(image).map_err(|_| "Not a disc image this can read".to_string())?;
    let files = disc.files();
    // The count tells a disc of something else from an image that couldn't
    // be read: that one has no files at all.
    let mut problem = format!("Not this game's disc ({} files)", files.len());
    'programs: for program in programs(game) {
        for module in program {
            let name = without_tag(module.file);
            let places: Vec<String> = std::iter::once(name.to_string()).chain(install_folders().iter().map(|folder| format!("{folder}/{name}"))).collect();
            let Some(entry) = places.iter().find_map(|place| files.iter().find(|entry| entry.path.eq_ignore_ascii_case(place))) else {
                continue 'programs;
            };
            let data = disc.read(entry).map_err(|_| "The disc image is damaged".to_string())?;
            let found: String = Sha256::digest(&data).iter().map(|byte| format!("{byte:02x}")).collect();
            log_hash(&format!("{} : {}", image.display(), entry.path), data.len(), &found);
            if found != module.sha256 {
                problem = "This disc is a different version".to_string();
                continue 'programs;
            }
        }
        return Ok(());
    }
    Err(problem)
}

/// Where a disc image is unpacked, and whether that's done already: `game`
/// beside the program, which is also where a start without any path looks
/// first, or a folder named for the version when `game` holds something else.
pub(crate) fn unpack_target(game: &Game, image: &std::path::Path) -> (std::path::PathBuf, bool) {
    let base = host::launcher::files_folder().unwrap_or_default();
    target_for(&base, &programs_on_image(game, image))
}

fn target_for(base: &std::path::Path, wanted: &[&[Module]]) -> (std::path::PathBuf, bool) {
    let plain = base.join("game");
    let Some(first) = wanted.first() else { return (plain, false) };
    // No dots: `disc::unpack` swaps the extension for its partial copy.
    let version: String = first[0].name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let named = base.join(format!("game-{version}"));
    // A copy made earlier stays in use, with its saves, if `game` has gone since.
    if let Some(copy) = [&plain, &named].into_iter().find(|folder| holds(folder, wanted)) {
        return (copy.clone(), true);
    }
    (if plain.exists() { named } else { plain }, false)
}

/// The folder an image was unpacked to earlier, if it was.
pub(crate) fn unpacked_copy(game: &Game, image: &std::path::Path) -> Option<std::path::PathBuf> {
    let (folder, done) = unpack_target(game, image);
    done.then_some(folder)
}

/// Unpacks a disc image where `unpack_target` says, unless that's done, and
/// returns the folder. `progress` gets the bytes written so far and the total.
pub(crate) fn unpack_image(game: &Game, image: &std::path::Path, progress: &mut dyn FnMut(u64, u64)) -> Result<String, String> {
    check_image(game, image)?;
    let (target, done) = unpack_target(game, image);
    if !done {
        if target.exists() {
            return Err(format!("{} is in the way: it holds other files. Move or delete it.", target.display()));
        }
        println!("unpacking {} into {}", image.display(), target.display());
        host::disc::unpack(image, &target, progress)?;
        choose_cd_music(image, &target);
    }
    // The image's music is played from the image, so the folder keeps where it is.
    host::cue::keep(image, &target);
    Ok(target.display().to_string())
}

/// The drive behind a read-only folder that is a mounted CD or DVD, read like a disc image.
pub(crate) fn disc_drive(folder: &std::path::Path) -> Option<std::path::PathBuf> {
    (folder.is_dir() && !host::disc::is_writable(folder)).then(|| host::disc::drive_of(folder)).flatten()
}

/// A game folder that can't be written to, such as a mounted CD or DVD, is copied into
/// `game` beside the program, since the game writes its saves and settings beside its
/// files. Gives the folder to use: the copy, or `folder` itself when it's writable or
/// doesn't hold the game.
pub(crate) fn copy_read_only(game: &Game, folder: &str, progress: &mut dyn FnMut(u64, u64)) -> Result<String, String> {
    let path = std::path::Path::new(folder);
    if !path.is_dir() || check_files(game, folder).is_err() || host::disc::is_writable(path) {
        return Ok(folder.to_string());
    }
    let target = host::launcher::files_folder().unwrap_or_default().join("game");
    let copy = target.display().to_string();
    if check_files(game, &copy).is_ok() {
        return Ok(copy);
    }
    if target.exists() {
        return Err(format!("{folder} can't be written to, and {copy} is in the way of a copy: it holds other files. Move or delete it."));
    }
    host::disc::copy_folder(path, &target, progress)?;
    Ok(copy)
}

/// The folder the game runs in for what the player chose: that folder, or a
/// disc image's unpacked copy. A folder that holds only an image stands for it.
pub(crate) fn playable_folder(game: &Game, chosen: &str, progress: &mut dyn FnMut(u64, u64)) -> Result<String, String> {
    let mut place = std::path::PathBuf::from(chosen);
    // A mounted CD or DVD whose game is somewhere on it stands for its drive.
    if check_files(game, chosen).is_err() {
        if let Some(drive) = disc_drive(&place).filter(|drive| check_image(game, drive).is_ok()) {
            place = drive;
        }
    }
    if place.is_dir() && check_files(game, chosen).is_err() {
        place = host::disc::image_in(&place).unwrap_or(place);
    }
    if !host::disc::is_image(&place) {
        return copy_read_only(game, chosen, progress);
    }
    unpack_image(game, &place, progress).map_err(|problem| {
        format!("{} can't use this disc image:\n{}\n\n{problem}\n\n{} needs {}.", game.title, place.display(), game.title, game.needs)
    })
}

/// After a disc image is unpacked: CD music from the image, rather than
/// MIDI, as the game's setting (see `cue::prefer_cd_music`). Only on a fresh
/// unpack, so a choice the player makes later stays.
pub(crate) fn choose_cd_music(image: &std::path::Path, target: &std::path::Path) {
    if let Some(config) = find_file(&target.display().to_string(), "INIT.CFG") {
        host::cue::prefer_cd_music(image, &config);
        use_controller(&config);
    }
}

/// The controller as the game's joystick from the start (see
/// `gamepad::game_port_read`): a 4-button joystick, calibrated to the
/// runtime's counts, so no calibration is needed. The disc's own controls
/// already put turning and height on the joystick. Only the named lines
/// change, and the file keeps its DOS line endings.
fn use_controller(config: &std::path::Path) {
    let calibrated = host::gamepad::port_calibration();
    let wanted = [
        ("JOYSTICKTYP", "4BUTTON".to_string()),
        // The game's presets (`NORMAL`, `PILOT`, `ARCADE`) bring their own
        // bindings, so the lines below only count with its own choice.
        ("STEERING", "INDIVIDUAL".to_string()),
        ("JOY1X", calibrated.clone()),
        ("JOY1Y", calibrated.clone()),
        ("JOY2X", calibrated.clone()),
        ("JOY2Y", calibrated),
        // The left stick flies: forward and back, and turning. The
        // triggers are the throttle, for height, and the right stick slides.
        ("FORBACK", "JOY@<NO_KEY>@Y_AXES_1_N".to_string()),
        ("ROTATE", "JOY@<NO_KEY>@X_AXES_1".to_string()),
        ("HEIGHT", "JOY@<NO_KEY>@Y_AXES_2_N".to_string()),
        ("SLIDE", "JOY@<NO_KEY>@X_AXES_2".to_string()),
        // Not the controller: the briefings as text and voice (Options'
        // "Briefing", 0 text, 1 both, 2 voice). The disc says 2, which leaves
        // every spoken line without its text box; an installed copy set up
        // on DOS (and the original in the user's recording) has 1.
        ("BRIEFING", "1".to_string()),
    ];
    let Ok(data) = std::fs::read(config) else { return };
    let text = String::from_utf8_lossy(&data);
    let changed: String = text
        .split_inclusive('\n')
        .map(|line| {
            let name = line.split('=').next().unwrap_or_default().trim();
            match wanted.iter().find(|(wanted, _)| name.eq_ignore_ascii_case(wanted)) {
                Some((name, value)) => format!("{name}={value}{}", if line.ends_with("\r\n") { "\r\n" } else if line.ends_with('\n') { "\n" } else { "" }),
                None => line.to_string(),
            }
        })
        .collect();
    if changed != text && std::fs::write(config, changed).is_ok() {
        println!("the controller is the joystick in {}", config.display());
    }
}

fn find_file(folder: &str, name: &str) -> Option<std::path::PathBuf> {
    // A name may hold folders, such as `Z/WINZ.EXE` on a copy of a disc. Each
    // part is matched without regard to case. On an unpacked disc, the file
    // may be in one of the folders the installer would have copied.
    let name = without_tag(name);
    let find = |name: &str| {
        let mut path = std::path::PathBuf::from(folder);
        for part in name.split('/') {
            path = std::fs::read_dir(&path).ok()?.flatten().find(|entry| entry.file_name().to_string_lossy().eq_ignore_ascii_case(part)).map(|entry| entry.path())?;
        }
        Some(path)
    };
    find(name).or_else(|| install_folders().iter().find_map(|install| find(&format!("{install}/{name}"))))
}

fn load_images(modules: &[Module], folder: &str) -> Vec<(String, pe::Image)> {
    modules
        .iter()
        .map(|module| {
            let path = find_file(folder, module.file).unwrap_or_else(|| panic!("{} is missing from {folder}", module.file));
            let file = std::fs::read(path).expect("a game file can't be read");
            let mut image = pe::Image::parse_any(&file).expect("a game file isn't a 32-bit Windows or DOS/4GW program");
            image.rebase(module.base);
            // Other modules import it by its name alone, whatever folder it's in.
            let file = without_tag(module.file);
            (file.rsplit('/').next().unwrap_or(file).to_lowercase(), image)
        })
        .collect()
}

fn resolve(image: &pe::Image, others: &[(String, pe::Image)], cpu: &mut Cpu, known: &mut HashMap<String, u32>) {
    for import in &image.imports {
        let label = match &import.name {
            Some(name) => format!("{}!{name}", import.dll),
            None => format!("{}!#{}", import.dll, import.ordinal),
        };
        let export = others.iter().find(|(file, _)| *file == import.dll).and_then(|(_, other)| {
            other.exports.iter().find(|e| match &import.name {
                Some(name) => e.name.as_deref() == Some(name),
                None => e.ordinal == import.ordinal,
            })
        });
        let address = match export {
            Some(export) => export.address,
            None => *known.entry(label.clone()).or_insert_with(|| {
                let key = import.name.clone().unwrap_or_else(|| format!("#{}", import.ordinal));
                let function = host::find(&import.dll, &key);
                cpu.imports.push(Host { name: label.clone(), function });
                IMPORT_BASE + cpu.imports.len() as u32 - 1
            }),
        };
        cpu.w32(import.slot, address);
    }
}

fn start(game: &'static Game, folder: String, switches: Vec<bool>, mods: Vec<bool>, command: &'static str) {
    let table: Vec<(u32, GuestFn)> = game.tables.iter().flat_map(|table| table.iter().copied()).collect();
    let images = load_images(program_in(game, &folder), &folder);
    host::cue::find(std::path::Path::new(&folder));
    let mut cpu = Cpu {
        eax: 0, ecx: 0, edx: 0, ebx: 0, esp: STACK_TOP - 0x100, ebp: 0, esi: 0, edi: 0, returned_to: 0,
        segments: [host::dos::FLAT; 6], bases: [0; 6],
        cf: false, zf: false, sf: false, of: false, df: false, parity: 1,
        fs_base: THREAD_BLOCK,
        st: [0.0; 8], top: 0, c0: false, c1: false, c2: false, c3: false, fpu_control: 0x27f,
        memory: crate::sys::reserve_memory(),
        functions: table.iter().copied().collect(),
        imports: Vec::new(),
        host: Box::new(host::State::new(&folder)),
        trace: std::env::var_os("RECOMP_TRACE").is_some(),
        trace_filter: std::env::var("RECOMP_TRACE").ok().filter(|value| value != "1"),
        watched: HashMap::new(),
        watch_counts: HashMap::new(),
        profiling: false,
        hits: HashMap::new(),
        error_function: None,
    };
    let errors = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("errors:").map(str::to_string));
    if let Some(address) = errors {
        cpu.host.script.steps.pop_front();
        cpu.error_function = u32::from_str_radix(&address, 16).ok();
        cpu.profiling = true;
    }
    // A script can name a DLL with a `watch:` first step, to log its exports.
    let watched = cpu.host.script.steps.front().and_then(|step| step.strip_prefix("watch:").map(str::to_string));
    if let Some(watched) = watched {
        cpu.host.script.steps.pop_front();
        for (name, image) in &images {
            if *name == watched.to_lowercase() {
                for export in &image.exports {
                    cpu.watched.insert(export.address, export.name.clone().unwrap_or_default());
                }
            }
        }
    }
    for (_, image) in &images {
        unsafe { std::ptr::copy_nonoverlapping(image.memory.as_ptr(), cpu.memory.add(image.base as usize), image.memory.len()) };
    }
    let mut known = HashMap::new();
    for (_, image) in &images {
        resolve(image, &images, &mut cpu, &mut known);
    }
    for ((_, address, _), on) in game.switches.iter().zip(switches) {
        // A switch at 0 went on the command line instead.
        if on && *address != 0 {
            cpu.w32(*address, 1);
        }
    }
    cpu.w32(THREAD_BLOCK, 0xffff_ffff); // No exception handlers yet.
    cpu.w32(THREAD_BLOCK + 4, STACK_TOP);
    cpu.w32(THREAD_BLOCK + 8, STACK_TOP - STACK_SIZE);
    cpu.w32(THREAD_BLOCK + 0x18, THREAD_BLOCK);

    println!("{} modules loaded, {} functions, {} imports for the runtime", images.len(), table.len(), cpu.imports.len());
    if let Ok(path) = std::env::var("RECOMP_IMPORTS") {
        let missing: Vec<&str> = cpu.imports.iter().filter(|host| host.function.is_none()).map(|host| host.name.as_str()).collect();
        let _ = std::fs::write(path, missing.join("\n"));
    }
    cpu.host.title = game.title;
    cpu.host.command = command;
    cpu.host.icon = game.icon;
    cpu.host.music = game.music;
    cpu.host.cd = game.cd;
    cpu.host.registry = host::registry::Registry::load(&cpu.host.folder, game.registry);
    cpu.host.mods_off = game.mods.iter().zip(mods).filter(|(_, on)| !on).map(|((file, ..), _)| *file).collect();
    let entry = images[0].1.entry;
    if images[0].1.is_dos() {
        // DOS's start: the PSP, the environment and the selectors, with the
        // program as `Z.BAT` and friends named it.
        let program = format!("C:\\{}", images[0].0.to_uppercase());
        // The words after the program's name, as a batch file such as
        // `LOWMEM.BAT` passed them (`forbidhires`).
        let line = std::env::var("RECOMP_COMMAND").unwrap_or_else(|_| command.to_string());
        let from_command = line.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
        let arguments = std::env::var("RECOMP_DOS_ARGUMENTS").ok().filter(|text| !text.is_empty()).unwrap_or(from_command);
        if !arguments.is_empty() {
            println!("{program} {arguments}");
        }
        // The program really started, which the renderer chose (XA.EXE or XA_3DFX.EXE).
        crate::runlog::line(&format!("started: {program} {arguments}"));
        host::dos::start(&mut cpu, images[0].1.stack, &program, &arguments);
    }
    for (name, image) in images {
        cpu.host.modules.push(host::LoadedModule {
            name,
            base: image.base,
            entry: image.entry,
            exports: image.exports,
            started: !image.is_dll, // The program's entry point runs last, below.
        });
    }
    for name in STARTED_FIRST {
        if let Some(index) = cpu.host.modules.iter().position(|module| module.name == *name) {
            cpu.start_module(index);
        }
    }
    cpu.push(0); // Returning from here ends the game.
    cpu.call(entry);
    // A program that ends normally exits through DOS or Windows. Returning
    // means a `ret` went somewhere unexpected and the frames unwound.
    println!("the game's entry point returned, to {:#x}, with esp {:#x} and eax {:#x}", cpu.returned_to, cpu.esp, cpu.eax);
}

/// What the player chose, on the command line or in the launcher.
pub struct Settings {
    pub folder: String,
    /// How many times 640x480 3D is drawn at, from 1 to 6.
    pub scale: u32,
    pub fullscreen: bool,
    /// Whether Glide is drawn by the processor instead of the graphics card.
    pub processor: bool,
    /// Whether settings, `recomp.log` and unpacked discs go to the XDG folders (Linux).
    pub xdg: bool,
    /// Whether later starts go straight into the game, past the launcher.
    pub skip_launcher: bool,
    /// One for each of the game's switches.
    pub switches: Vec<bool>,
    /// An index into the game's renderers.
    pub renderer: usize,
    /// An index into the game's languages.
    pub language: usize,
    /// One for each of the game's optional mods: whether it's on.
    pub mods: Vec<bool>,
}

/// The renderer or language named by a label or a switch, ignoring case.
pub fn find_choice(choices: &[(&str, &str)], name: &str) -> Option<usize> {
    choices.iter().position(|(switch, label)| label.eq_ignore_ascii_case(name) || switch.eq_ignore_ascii_case(name))
}

/// The game's command line, with the chosen renderer's and language's switches in place of
/// any others from those lists, and the switches the game reads itself that are on. A
/// renderer that is a program of its own is the command's first word, its program.
fn command_with_choices(game: &Game, settings: &Settings) -> String {
    let lists = [(game.renderers, settings.renderer), (game.languages, settings.language)];
    let listed = |word: &str| lists.iter().any(|(list, _)| list.iter().any(|(switch, _)| switch.eq_ignore_ascii_case(word)));
    let mut words: Vec<&str> = game.command.split_whitespace().filter(|word| !listed(word)).collect();
    for (number, (list, index)) in lists.into_iter().enumerate() {
        if let Some((chosen, _)) = list.get(index) {
            if number == 0 && renderer_program(game, index).is_some() {
                words.insert(0, chosen);
            } else {
                words.push(chosen);
            }
        }
    }
    for ((name, address, _), on) in game.switches.iter().zip(&settings.switches) {
        if *on && *address == 0 {
            words.push(name);
        }
    }
    words.join(" ")
}

pub const DEFAULT_SCALE: u32 = 3;

/// Reads `[folder] [--scale=N] [--fullscreen] [--drawing=processor|card] [--renderer=NAME] [--language=NAME] [--mod-off=NAME]
/// [--mod-on=NAME] [--add-patch=FILE] [--play] [game switches]`.
fn settings_from_arguments(game: &Game, arguments: &[String]) -> Settings {
    let has = |wanted: &str| arguments.iter().any(|argument| argument.eq_ignore_ascii_case(wanted));
    let scale = arguments.iter().find_map(|argument| argument.strip_prefix("--scale=")?.parse().ok());
    let scale = scale.or_else(|| std::env::var("RECOMP_SCALE").ok()?.parse().ok());
    Settings {
        folder: arguments.iter().find(|argument| !argument.starts_with("--")).cloned().unwrap_or_else(|| default_folder(game)),
        scale: scale.unwrap_or(DEFAULT_SCALE).clamp(1, 6),
        fullscreen: has("--fullscreen"),
        processor: has("--drawing=processor"),
        xdg: host::launcher::xdg_on(),
        skip_launcher: false,
        switches: game.switches.iter().map(|(name, _, _)| has(name)).collect(),
        renderer: arguments.iter().find_map(|argument| find_choice(game.renderers, argument.strip_prefix("--renderer=")?)).unwrap_or(0),
        language: arguments.iter().find_map(|argument| find_choice(game.languages, argument.strip_prefix("--language=")?)).unwrap_or(0),
        mods: game.mods.iter().map(|(file, _, on, _)| {
            let named = |prefix: &str| arguments.iter().any(|argument| argument.strip_prefix(prefix).is_some_and(|name| name.eq_ignore_ascii_case(file)));
            if named("--mod-off=") { false } else if named("--mod-on=") { true } else { *on }
        }).collect(),
    }
}

/// Runs the game on a thread with a host stack deep enough for its calls.
pub fn run(game: &'static Game) {
    host::launcher::choose_folders(game, &std::env::args().skip(1).collect::<Vec<String>>());
    crate::runlog::start(game.title, game.version, game.normal_exits, || host::launcher::settings_folder(game));
    report_crashes(game.title);
    let _ = INSTALL_FOLDERS.set(game.install);
    let _ = CD_TRACKS.set(game.cd_tracks);
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut settings = settings_from_arguments(game, &arguments);
    // A test's own command line (`./port recomp NAME STEPS "" "XA.EXE forbidhires"`) picks
    // the renderer that its first word names.
    let test_command = std::env::var("RECOMP_COMMAND").ok();
    if let Some(index) = test_command.as_deref().and_then(|command| find_choice(game.renderers, command.split_whitespace().next()?)) {
        settings.renderer = index;
    }
    choose_renderer(game, settings.renderer);
    host::launcher::snapshot(game, &settings);
    // The port's keys as the player set them, with or without the launcher.
    host::launcher::load_hotkeys(game);
    if host::launcher::wanted(&arguments) {
        // The launcher guesses no folder: it shows the one saved last time, or none to choose.
        if !arguments.iter().any(|argument| !argument.starts_with("--")) {
            settings.folder = String::new();
        }
        // A player starts in fullscreen unless they chose a window before
        // (the saved setting wins) or asked for one with `--window`.
        settings.fullscreen = !arguments.iter().any(|argument| argument == "--window");
        if host::launcher::skipped(game, &arguments, &mut settings) {
            // Straight into the game with the saved settings.
        } else {
            match host::launcher::show(game, settings) {
                host::launcher::Outcome::Start(chosen) => settings = chosen,
                host::launcher::Outcome::Quit => std::process::exit(0),
                host::launcher::Outcome::Unavailable(unchanged) => settings = unchanged,
            }
        }
    }
    // The graphics code reads these when the game opens its window.
    std::env::set_var("RECOMP_SCALE", settings.scale.to_string());
    if settings.fullscreen {
        std::env::set_var("RECOMP_FULLSCREEN", "1");
    }
    // The player's own RECOMP_GLIDE wins, as it does for tests.
    if settings.processor && std::env::var_os("RECOMP_GLIDE").is_none() {
        std::env::set_var("RECOMP_GLIDE", "cpu");
    }
    // The launcher has unpacked a chosen image already; one given on the command line is unpacked here.
    let folder = playable_folder(game, &settings.folder, &mut |_, _| {}).unwrap_or_else(|text| {
        host::video::show_error(game.title, &text);
        std::process::exit(1);
    });
    // `--add-patch=FILE`: Extreme Assault's German 3dfx patch (`xa_r110.zip`), as the
    // launcher adds it. A relative FILE may also be in the game folder, as in tests.
    if let Some(file) = arguments.iter().find_map(|argument| argument.strip_prefix("--add-patch=")) {
        let path = std::path::Path::new(file);
        let path = if path.exists() { path.to_path_buf() } else { std::path::Path::new(&folder).join(file) };
        let added = host::rtpatch::patch_program(&path).ok_or_else(|| format!("{file} isn't the German 3dfx patch (xa_r110.zip)"))
            .and_then(|patch| host::rtpatch::apply_to(&folder, &patch));
        match added {
            Ok(written) => println!("3dfx patch added: {written} files written"),
            Err(problem) => {
                host::video::show_error(game.title, &problem);
                std::process::exit(1);
            }
        }
    }
    // A renderer the folder lacks gives way to one it has: the German disc has only software.
    settings.renderer = usable_renderer(game, &folder, settings.renderer);
    choose_renderer(game, settings.renderer);
    let command: &'static str = Box::leak(command_with_choices(game, &settings).into_boxed_str());
    log_settings(game, &settings, &folder, command);
    let Settings { switches, mods, .. } = settings;
    if let Err(text) = check_files(game, &folder) {
        host::video::show_error(game.title, &text);
        std::process::exit(1);
    }
    // The installer makes SAVEGAME, and a disc doesn't hold one. Without it the
    // game can't write its saves. A read-only folder, as in tests, keeps none.
    let has_saves = std::fs::read_dir(&folder).is_ok_and(|mut entries| entries.any(|entry| entry.is_ok_and(|entry| entry.file_name().eq_ignore_ascii_case("SAVEGAME"))));
    if !game.install.is_empty() && !has_saves {
        let _ = std::fs::create_dir(std::path::Path::new(&folder).join("SAVEGAME"));
    }
    let thread = std::thread::Builder::new().stack_size(HOST_STACK).spawn(move || start(game, folder, switches, mods, command));
    if thread.expect("couldn't start the game's thread").join().is_err() {
        crate::runlog::line("the game exited with code 1 after the error above");
        std::process::exit(1);
    }
    crate::runlog::line("the game exited with code 0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_image_only_reuses_its_own_version() {
        let base = std::env::temp_dir().join("loader-test-image-version");
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let module = |name: &'static str, contents: &str| {
            let sha256 = Sha256::digest(contents.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect::<String>();
            [Module { file: "XA.EXE", base: 0, sha256: Box::leak(sha256.into_boxed_str()), program: true, name }]
        };
        let (english, german) = (module("English CD 1.1.0", "english"), module("German CD 1.0.1", "german"));
        let (plain, named) = (base.join("game"), base.join("game-german-cd-1-0-1"));

        assert_eq!(target_for(&base, &[&german]), (plain.clone(), false));
        std::fs::create_dir_all(&plain).unwrap();
        std::fs::write(plain.join("XA.EXE"), "english").unwrap();
        assert_eq!(target_for(&base, &[&english]), (plain.clone(), true));
        assert_eq!(target_for(&base, &[&german]), (named.clone(), false));
        std::fs::create_dir_all(&named).unwrap();
        std::fs::write(named.join("XA.EXE"), "german").unwrap();
        assert_eq!(target_for(&base, &[&german]), (named.clone(), true));
        std::fs::remove_dir_all(&plain).unwrap();
        assert_eq!(target_for(&base, &[&german]), (named, true));
        let _ = std::fs::remove_dir_all(&base);
    }
}
