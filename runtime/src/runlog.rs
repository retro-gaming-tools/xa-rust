//! `recomp.log`: what a run chose and printed, and how it ended, for a player to send.
//! The program has no console on Windows, so this file is the only record of a run.
//! It is written anew each run beside the program, or in the launcher's settings
//! folder when the program's folder can't be written (an AppImage's is read-only).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub const FILE_NAME: &str = "recomp.log";
/// How many lines of the game's output the exit message shows.
const TAIL_LINES: usize = 6;
/// The output kept in memory for that message.
const KEPT_LINES: usize = 64;

struct Log {
    file: Option<std::fs::File>,
    path: Option<PathBuf>,
    /// The last lines printed, and the line not yet ended.
    lines: std::collections::VecDeque<String>,
    partial: String,
}

static LOG: Mutex<Log> = Mutex::new(Log { file: None, path: None, lines: std::collections::VecDeque::new(), partial: String::new() });
static TITLE: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
/// Exit codes besides 0 that mean the player quit (`recompNormalExits`).
static NORMAL_EXITS: std::sync::OnceLock<&'static [i32]> = std::sync::OnceLock::new();

/// Whether an exit code is an error worth telling the player about.
pub fn is_error(code: i32, normal: &[i32]) -> bool {
    code != 0 && !normal.contains(&code)
}

/// The log of the run before, kept when a new run starts, since a player who
/// starts the game again after an error would otherwise lose the log to send.
pub const PREVIOUS_FILE_NAME: &str = "recomp-previous.log";

fn log() -> std::sync::MutexGuard<'static, Log> {
    LOG.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The first of the folders where `recomp.log` can be created, and that file opened.
pub fn open_in(folders: &[PathBuf]) -> Option<(PathBuf, std::fs::File)> {
    folders.iter().find_map(|folder| {
        let path = folder.join(FILE_NAME);
        if path.is_file() {
            let _ = std::fs::rename(&path, folder.join(PREVIOUS_FILE_NAME));
        }
        std::fs::File::create(&path).ok().map(|file| (path, file))
    })
}

/// Starts the log: beside the program, else in `fallback` (the launcher's settings folder).
pub fn start(title: &'static str, version: &str, normal_exits: &'static [i32], fallback: impl FnOnce() -> Option<PathBuf>) {
    let _ = TITLE.set(title);
    let _ = NORMAL_EXITS.set(normal_exits);
    // Beside the `.AppImage`, as settings.ini is, not in its read-only mount.
    let mut folders: Vec<PathBuf> = crate::host::launcher::program_folder().into_iter().collect();
    let opened = open_in(&folders).or_else(|| {
        folders = fallback().into_iter().collect();
        open_in(&folders)
    });
    let mut log = log();
    if let Some((path, file)) = opened {
        log.file = Some(file);
        log.path = Some(path);
    }
    drop(log);
    line(&format!("{title} {version}, on {} {}", std::env::consts::OS, std::env::consts::ARCH));
}

/// Where the log is, if it could be written.
pub fn path() -> Option<PathBuf> {
    log().path.clone()
}

/// Adds text to the log as it is printed. Lines are kept for the exit message.
pub fn write(text: &str) {
    let mut log = log();
    if let Some(file) = log.file.as_mut() {
        let _ = file.write_all(text.as_bytes());
    }
    let mut partial = std::mem::take(&mut log.partial);
    partial.push_str(text);
    let mut rest = partial.as_str();
    while let Some(end) = rest.find('\n') {
        let done = rest[..end].trim_end_matches('\r').to_string();
        log.lines.push_back(done);
        if log.lines.len() > KEPT_LINES {
            log.lines.pop_front();
        }
        rest = &rest[end + 1..];
    }
    log.partial = rest.to_string();
}

/// For the panic hook: a panic while the log is held mustn't wait on itself.
pub fn write_panic(text: &str) {
    if let Ok(mut log) = LOG.try_lock() {
        if let Some(file) = log.file.as_mut() {
            let _ = file.write_all(text.as_bytes());
        }
    }
}

pub fn line(text: &str) {
    write(&format!("{text}\n"));
}

/// The last `count` lines that held something, with a line not yet ended.
pub fn tail(lines: &[String], partial: &str, count: usize) -> Vec<String> {
    let mut all: Vec<String> = lines.iter().filter(|line| !line.trim().is_empty()).cloned().collect();
    if !partial.trim().is_empty() {
        all.push(partial.to_string());
    }
    let skip = all.len().saturating_sub(count);
    all.split_off(skip)
}

/// The message shown when the game stops with an error code.
pub fn exit_message(title: &str, code: i32, tail: &[String], log: Option<&Path>) -> String {
    let mut text = format!("{title} stopped with error code {code}.");
    if !tail.is_empty() {
        text.push_str("\n\nIts last words:\n");
        text.push_str(&tail.join("\n"));
    }
    match log {
        Some(path) => text.push_str(&format!(
            "\n\nPlease send this file with your report:\n{}\n(Starting the game again keeps it as {PREVIOUS_FILE_NAME}.)",
            path.display()
        )),
        None => text.push_str("\n\nThe log recomp.log couldn't be written."),
    }
    text
}

/// A scripted or headless run (tests, zone checks) must never wait on a message box.
pub fn automated() -> bool {
    std::env::var("RECOMP_SCRIPT").is_ok_and(|script| !script.trim().is_empty()) || std::env::var_os("RECOMP_HEADLESS").is_some()
}

/// Ends the process for the guest: logs the code, and tells the player about an error.
pub fn exit(code: i32) -> ! {
    std::println!("the game exited with code {code}");
    line(&format!("the game exited with code {code}"));
    if is_error(code, NORMAL_EXITS.get().copied().unwrap_or(&[])) && !automated() {
        let (lines, partial, path) = {
            let log = log();
            (log.lines.iter().cloned().collect::<Vec<_>>(), log.partial.clone(), log.path.clone())
        };
        let lines: Vec<String> = lines.into_iter().filter(|line| !line.starts_with("the game exited with code")).collect();
        let title = TITLE.get().copied().unwrap_or("The game");
        let text = exit_message(title, code, &tail(&lines, &partial, TAIL_LINES), path.as_deref());
        crate::host::video::show_error(title, &text);
    }
    std::process::exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_falls_back_to_a_writable_folder() {
        let base = std::env::temp_dir().join(format!("recomp-log-test-{}", std::process::id()));
        let writable = base.join("settings");
        std::fs::create_dir_all(&writable).unwrap();
        let missing = base.join("no-such-folder");
        let (path, _) = open_in(&[missing.clone(), writable.clone()]).unwrap();
        assert_eq!(path, writable.join(FILE_NAME));
        // A second run keeps the first one's log as the previous log.
        std::fs::write(&path, "first run").unwrap();
        open_in(&[missing, writable.clone()]).unwrap();
        assert_eq!(std::fs::read_to_string(writable.join(PREVIOUS_FILE_NAME)).unwrap(), "first run");
        assert!(open_in(&[base.join("also-missing")]).is_none());
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn normal_quit_codes_are_not_errors() {
        assert!(!is_error(0, &[]));
        assert!(!is_error(39, &[39]));
        assert!(is_error(71, &[39]));
        assert!(is_error(39, &[]));
    }

    #[test]
    fn tail_keeps_the_last_lines_with_text() {
        let lines: Vec<String> = ["one", "", "two", "three"].iter().map(|line| line.to_string()).collect();
        assert_eq!(tail(&lines, "four", 2), vec!["three", "four"]);
        assert_eq!(tail(&lines, " ", 5), vec!["one", "two", "three"]);
    }

    #[test]
    fn exit_message_names_the_code_output_and_log() {
        let text = exit_message("Extreme Assault", 71, &["INIT:: error parsing".to_string()], Some(Path::new("/x/recomp.log")));
        assert!(text.starts_with("Extreme Assault stopped with error code 71."));
        assert!(text.contains("INIT:: error parsing"));
        assert!(text.contains("/x/recomp.log\n"));
        assert!(text.contains(PREVIOUS_FILE_NAME));
        assert!(exit_message("X", 1, &[], None).contains("couldn't be written"));
    }
}
