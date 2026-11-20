//! A disc's `.cue` sheet: where each track starts on the disc, and where an
//! audio track's sound is in the image's files, so CD music can play straight
//! from the player's image. Audio sectors are 2352 bytes of 16-bit stereo at
//! 44.1 kHz, so nothing needs decoding.
//!
//! Only `BINARY` files are read. A sheet that names others, such as Ogg files
//! for each track, gives no layout here, because their lengths aren't known.

use std::path::{Path, PathBuf};

pub const AUDIO_SECTOR: u64 = 2352;
const PER_SECOND: u32 = 75;

/// Where one audio track's sound is: a file, its first byte and its length.
#[derive(Clone, Debug, PartialEq)]
pub struct Sound {
    pub file: PathBuf,
    pub offset: u64,
    pub bytes: u64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Sheet {
    /// Where each track starts, in sectors from the start of the disc, track
    /// 1 first, and then where the disc ends.
    pub starts: Vec<u32>,
    /// Each track's sound, track 1 first: none for a data track.
    pub sounds: Vec<Option<Sound>>,
}

fn sector_size(mode: &str) -> u64 {
    match mode.to_ascii_uppercase().as_str() {
        "MODE1/2048" => 2048,
        "MODE2/2336" => 2336,
        _ => AUDIO_SECTOR,
    }
}

fn sectors_of(place: &str) -> Option<u32> {
    let parts: Vec<u32> = place.split(':').map(|part| part.parse().ok()).collect::<Option<_>>()?;
    match parts[..] {
        [minutes, seconds, frames] if seconds < 60 && frames < PER_SECOND => Some((minutes * 60 + seconds) * PER_SECOND + frames),
        _ => None,
    }
}

/// One track while the sheet is read.
struct Pending {
    file: usize,
    mode: String,
    pregap: u32,
    index0: Option<u32>,
    index1: Option<u32>,
}

/// Reads a sheet. `size` gives a file's length, so the last track in each
/// file knows where it ends. `None` when the sheet isn't one this can use.
pub fn parse(text: &str, folder: &Path, size: &dyn Fn(&Path) -> Option<u64>) -> Option<Sheet> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut tracks: Vec<Pending> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        let Some(word) = line.split_whitespace().next() else { continue };
        match word.to_ascii_uppercase().as_str() {
            "FILE" => {
                let name = line.split('"').nth(1).map(str::to_string).or_else(|| line.split_whitespace().nth(1).map(str::to_string))?;
                if !line.split_whitespace().last()?.eq_ignore_ascii_case("BINARY") {
                    return None;
                }
                let path = Path::new(&name);
                files.push(if path.is_absolute() { path.to_path_buf() } else { folder.join(path) });
            }
            "TRACK" => {
                let mode = line.split_whitespace().nth(2)?.to_string();
                tracks.push(Pending { file: files.len().checked_sub(1)?, mode, pregap: 0, index0: None, index1: None });
            }
            "PREGAP" => tracks.last_mut()?.pregap = sectors_of(line.split_whitespace().nth(1)?)?,
            "INDEX" => {
                let mut rest = line.split_whitespace().skip(1);
                let (number, place) = (rest.next()?, sectors_of(rest.next()?)?);
                let track = tracks.last_mut()?;
                match number.parse::<u32>().ok()? {
                    0 => track.index0 = Some(place),
                    1 => track.index1 = Some(place),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    if tracks.is_empty() || tracks.iter().any(|track| track.index1.is_none()) {
        return None;
    }
    // Each file's length in sectors, from the first track in it.
    let mut file_sectors = Vec::new();
    for (number, file) in files.iter().enumerate() {
        let mode = &tracks.iter().find(|track| track.file == number)?.mode;
        file_sectors.push((size(file)? / sector_size(mode)) as u32);
    }
    let mut sheet = Sheet::default();
    // Where the current file starts on the disc, and the pregaps so far in
    // it, which take up the disc but aren't in the file.
    let (mut file_start, mut gaps, mut file) = (0u32, 0u32, 0usize);
    for (number, track) in tracks.iter().enumerate() {
        while file < track.file {
            file_start += file_sectors[file] + gaps;
            gaps = 0;
            file += 1;
        }
        // A `PREGAP` is silence the image leaves out.
        gaps += track.pregap;
        let index1 = track.index1?;
        sheet.starts.push(file_start + gaps + index1);
        // The sound runs to where the next track in the same file starts,
        // its pause included, or else to the end of the file.
        let next = tracks.get(number + 1).filter(|next| next.file == track.file);
        let end = next.map_or(file_sectors[track.file], |next| next.index0.or(next.index1).unwrap_or(index1));
        let audio = track.mode.eq_ignore_ascii_case("AUDIO");
        sheet.sounds.push(audio.then(|| Sound {
            file: files[track.file].clone(),
            offset: index1 as u64 * AUDIO_SECTOR,
            bytes: end.saturating_sub(index1) as u64 * AUDIO_SECTOR,
        }));
    }
    while file < file_sectors.len() {
        file_start += file_sectors[file] + gaps;
        gaps = 0;
        file += 1;
    }
    sheet.starts.push(file_start);
    sheet.starts.windows(2).all(|pair| pair[0] < pair[1]).then_some(sheet)
}

/// Reads a sheet from disk.
pub fn load(path: &Path) -> Option<Sheet> {
    let text = std::fs::read(path).ok()?;
    let folder = path.parent().unwrap_or(Path::new("."));
    parse(&String::from_utf8_lossy(&text), folder, &|file| std::fs::metadata(file).ok().map(|m| m.len()))
}

/// Makes CD music the game's choice in a freshly unpacked disc, when the
/// image has music to play: Extreme Assault's disc ships `INIT.CFG` with
/// `MUSIC=MIDI`, which its `SETUP.EXE` changed on a real PC, and MIDI has no
/// player here. `config` is the unpacked `INIT.CFG`. Only the `MUSIC=` line
/// changes, and the file keeps its DOS line endings, which the game needs.
pub fn prefer_cd_music(image: &Path, config: &Path) {
    let is_cue = image.extension().is_some_and(|e| e.eq_ignore_ascii_case("cue"));
    let sheet = load(&if is_cue { image.to_path_buf() } else { image.with_extension("cue") });
    if !sheet.is_some_and(|sheet| sheet.sounds.iter().any(Option::is_some)) {
        return;
    }
    let Ok(data) = std::fs::read(config) else { return };
    let text = String::from_utf8_lossy(&data);
    if !text.lines().any(|line| line.trim().eq_ignore_ascii_case("MUSIC=MIDI")) {
        return;
    }
    let changed: String = text
        .split_inclusive('\n')
        .map(|line| if line.trim().eq_ignore_ascii_case("MUSIC=MIDI") { line.replacen(line.trim(), "MUSIC=CD_AUDIO", 1) } else { line.to_string() })
        .collect();
    if std::fs::write(config, changed).is_ok() {
        println!("CD music chosen in {}", config.display());
    }
}

/// The name of the sheet an unpacked disc keeps in its folder.
pub const KEPT: &str = "disc.cue";

static DISC: std::sync::OnceLock<Option<Sheet>> = std::sync::OnceLock::new();

/// Finds the disc for a run in the game's folder: the sheet an unpacked image
/// kept there, or else a sheet beside the game's files, such as a `.cue` and
/// `.bin` pair copied into it.
pub fn find(folder: &Path) {
    let kept = std::iter::once(folder.join(KEPT));
    let others = std::fs::read_dir(folder).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().is_some_and(|e| e.eq_ignore_ascii_case("cue")));
    let mut sheets: Vec<PathBuf> = kept.chain(others).filter(|path| path.is_file()).collect();
    sheets.dedup();
    let sheet = sheets.iter().find_map(|path| load(path).filter(|sheet| sheet.sounds.iter().any(Option::is_some)));
    if let Some(sheet) = &sheet {
        println!("CD music from {}", sheet.sounds.iter().flatten().next().map_or(String::new(), |sound| sound.file.display().to_string()));
    }
    let _ = DISC.set(sheet);
}

/// The disc found for this run, if any.
pub fn disc() -> Option<&'static Sheet> {
    DISC.get().and_then(Option::as_ref)
}

/// Keeps a copy of an image's sheet in the folder it was unpacked to, with
/// its files named by their full paths, so the music on the image can still
/// be played once the game runs from that folder. Does nothing for an image
/// with no usable sheet, or no audio.
pub fn keep(image: &Path, folder: &Path) {
    let is_cue = image.extension().is_some_and(|e| e.eq_ignore_ascii_case("cue"));
    let sheet_path = if is_cue { image.to_path_buf() } else { image.with_extension("cue") };
    let Ok(text) = std::fs::read(&sheet_path) else { return };
    let text = String::from_utf8_lossy(&text);
    let Some(sheet) = load(&sheet_path) else { return };
    if sheet.sounds.iter().all(Option::is_none) {
        return;
    }
    let image_folder = std::fs::canonicalize(sheet_path.parent().unwrap_or(Path::new("."))).unwrap_or_default();
    let mut kept = String::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let named = trimmed.get(..4).is_some_and(|word| word.eq_ignore_ascii_case("FILE"));
        match line.split('"').nth(1).filter(|_| named) {
            Some(name) => {
                let full = image_folder.join(name);
                let rest = line.rsplit('"').next().unwrap_or(" BINARY");
                kept.push_str(&format!("FILE \"{}\"{rest}\n", full.display()));
            }
            None => {
                kept.push_str(line);
                kept.push('\n');
            }
        }
    }
    let _ = std::fs::write(folder.join(KEPT), kept);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes(sectors: &'static [(&'static str, u64)]) -> impl Fn(&Path) -> Option<u64> {
        move |path| sectors.iter().find(|(name, _)| path.ends_with(name)).map(|(_, bytes)| *bytes)
    }

    #[test]
    fn one_file_with_a_pregap() {
        let text = "FILE \"a.bin\" BINARY\n  TRACK 01 MODE1/2352\n    INDEX 01 00:00:00\n  TRACK 02 AUDIO\n    PREGAP 00:02:00\n    INDEX 01 00:10:00\n  TRACK 03 AUDIO\n    INDEX 00 00:20:00\n    INDEX 01 00:22:00\n";
        let sheet = parse(text, Path::new("/d"), &sizes(&[("a.bin", 3000 * AUDIO_SECTOR)])).unwrap();
        assert_eq!(sheet.starts, vec![0, 150 + 750, 150 + 1650, 150 + 3000]);
        assert_eq!(sheet.sounds[0], None);
        assert_eq!(sheet.sounds[1], Some(Sound { file: "/d/a.bin".into(), offset: 750 * AUDIO_SECTOR, bytes: 750 * AUDIO_SECTOR }));
        assert_eq!(sheet.sounds[2].as_ref().unwrap().bytes, (3000 - 1650) * AUDIO_SECTOR);
    }

    #[test]
    fn a_file_per_track() {
        let text = "FILE \"1.bin\" BINARY\n TRACK 01 MODE1/2048\n  INDEX 01 00:00:00\nFILE \"2.bin\" BINARY\n TRACK 02 AUDIO\n  INDEX 00 00:00:00\n  INDEX 01 00:02:00\n";
        let sheet = parse(text, Path::new("/d"), &sizes(&[("1.bin", 1000 * 2048), ("2.bin", 900 * AUDIO_SECTOR)])).unwrap();
        assert_eq!(sheet.starts, vec![0, 1150, 1900]);
        assert_eq!(sheet.sounds[1].as_ref().unwrap().offset, 150 * AUDIO_SECTOR);
        assert_eq!(sheet.sounds[1].as_ref().unwrap().bytes, 750 * AUDIO_SECTOR);
    }

    #[test]
    fn other_files_and_broken_sheets_give_nothing() {
        let size = sizes(&[("a.bin", 100 * AUDIO_SECTOR)]);
        assert_eq!(parse("FILE \"t.ogg\" MP3\n TRACK 01 AUDIO\n  INDEX 01 00:00:00\n", Path::new("/d"), &size), None);
        assert_eq!(parse("TRACK 01 AUDIO\n", Path::new("/d"), &size), None);
        assert_eq!(parse("FILE \"a.bin\" BINARY\n TRACK 01 AUDIO\n  INDEX 01 00:99:00\n", Path::new("/d"), &size), None);
        assert_eq!(parse("FILE \"missing.bin\" BINARY\n TRACK 01 AUDIO\n  INDEX 01 00:00:00\n", Path::new("/d"), &size), None);
    }
}
