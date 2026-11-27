//! The launcher from a picked file to the folder that runs (user, 2026-11-22: a
//! German image started the English game's Glide, and its path wasn't saved).
//! The discs are made here and hold no game data.

use super::*;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const ENGLISH: &[(&str, &str)] = &[("XA_3DFX.EXE", "english glide"), ("XA.EXE", "english software")];
const GERMAN: &[(&str, &str)] = &[("XA.EXE", "german software")];
const GERMAN_GLIDE: &str = "german glide";
const GLIDE: usize = 0;
const SOFTWARE: usize = 1;

struct Player {
    game: Game,
    root: PathBuf,
    beside: PathBuf,
}

impl Player {
    fn new(test: &str) -> Player {
        let root = std::env::temp_dir().join(format!("launcher-test-{}-{test}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let beside = root.join("program");
        std::fs::create_dir_all(&beside).unwrap();
        TEST_PROGRAM_FOLDER.with(|folder| *folder.borrow_mut() = Some(beside.clone()));
        let module = |file: &'static str, contents: &str, name: &'static str| {
            let sha256: String = Sha256::digest(contents.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
            loader::Module { file, base: 0, sha256: Box::leak(sha256.into_boxed_str()), program: true, name }
        };
        let modules = vec![
            module("XA_3DFX.EXE@1.1", ENGLISH[0].1, "English CD 1.1.0"),
            module("XA.EXE@1.1", ENGLISH[1].1, "English CD 1.1.0"),
            module("XA.EXE@de", GERMAN[0].1, "German CD 1.0.1"),
            module("XA_3DFX.EXE@de", GERMAN_GLIDE, "German 3dfx patch 1.1.0"),
        ];
        let game = Game {
            title: "Test",
            version: "0",
            needs: "a test disc",
            command: "XA_3DFX.EXE",
            switches: &[],
            renderers: &[("XA_3DFX.EXE", "Glide"), ("XA.EXE", "Software")],
            languages: &[],
            mods: &[],
            modules: Box::leak(modules.into_boxed_slice()),
            tables: &[],
            icon: &[],
            music: &[],
            registry: &[],
            cd: "",
            install: &[],
            cd_tracks: &[],
            normal_exits: &[],
        };
        Player { game, root, beside }
    }

    /// A folder with a `.cue` and the ISO 9660 `.bin` it names; returns the `.cue`.
    fn disc(&self, name: &str, files: &[(&str, &str)]) -> PathBuf {
        const SECTOR: usize = 2048;
        fn record(name: &[u8], sector: usize, size: usize, flags: u8) -> Vec<u8> {
            let mut record = vec![0u8; (33 + name.len() + 1) & !1];
            record[0] = record.len() as u8;
            record[2..6].copy_from_slice(&(sector as u32).to_le_bytes());
            record[10..14].copy_from_slice(&(size as u32).to_le_bytes());
            record[25] = flags;
            record[32] = name.len() as u8;
            record[33..33 + name.len()].copy_from_slice(name);
            record
        }
        let mut image = vec![0u8; (19 + files.len()) * SECTOR];
        image[16 * SECTOR + 1..16 * SECTOR + 6].copy_from_slice(b"CD001");
        let top = record(&[0], 18, SECTOR, 2);
        image[16 * SECTOR + 156..16 * SECTOR + 156 + top.len()].copy_from_slice(&top);
        let mut at = 18 * SECTOR;
        for (number, (file, contents)) in files.iter().enumerate() {
            let entry = record(format!("{file};1").as_bytes(), 19 + number, contents.len(), 0);
            image[at..at + entry.len()].copy_from_slice(&entry);
            at += entry.len();
            let data = (19 + number) * SECTOR;
            image[data..data + contents.len()].copy_from_slice(contents.as_bytes());
        }
        let folder = self.root.join(name);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join(format!("{name}.bin")), image).unwrap();
        let cue = folder.join(format!("{name}.cue"));
        std::fs::write(&cue, format!("FILE \"{name}.bin\" BINARY\n  TRACK 01 MODE1/2048\n    INDEX 01 00:00:00\n")).unwrap();
        cue
    }

    fn folder(&self, path: &Path, files: &[(&str, &str)]) {
        std::fs::create_dir_all(path).unwrap();
        for (file, contents) in files {
            std::fs::write(path.join(file), contents).unwrap();
        }
    }

    fn settings(&self, folder: &Path) -> Settings {
        Settings {
            folder: folder.display().to_string(),
            scale: 3,
            fullscreen: false,
            processor: false,
            xdg: false,
            skip_launcher: false,
            switches: Vec::new(),
            renderer: GLIDE,
            language: 0,
            mods: Vec::new(),
        }
    }

    fn pick(&self, settings: &mut Settings, path: &Path) -> (String, bool) {
        choose_game(&self.game, settings, &path.display().to_string())
    }

    /// Start, as the launcher does it and then the loader; returns the folder that runs.
    fn start(&self, settings: &Settings) -> PathBuf {
        prepare_start(&self.game, settings, &mut |_, _| {}).unwrap();
        save(&self.game, settings);
        PathBuf::from(loader::playable_folder(&self.game, &settings.folder, &mut |_, _| {}).unwrap())
    }

    fn next_launch(&self) -> Settings {
        let mut settings = self.settings(Path::new(""));
        load(&self.game, &mut settings, false);
        settings.renderer = loader::usable_renderer(&self.game, &settings.folder, settings.renderer);
        settings
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn contents(folder: &Path, file: &str) -> String {
    std::fs::read_to_string(folder.join(file)).unwrap()
}

#[test]
fn german_image_over_an_english_game_runs_german_software() {
    let player = Player::new("reported");
    let english = player.beside.join("game");
    player.folder(&english, ENGLISH);
    let german = player.disc("german", GERMAN);
    let mut settings = player.settings(&english);

    let status = player.pick(&mut settings, &german);
    assert_eq!(status, ("Found: German CD 1.0.1, disc image".to_string(), true));
    assert_eq!(settings.renderer, SOFTWARE);
    let runs = player.start(&settings);
    assert_eq!(runs, player.beside.join("game-german-cd-1-0-1"));
    assert_eq!(contents(&runs, "XA.EXE"), GERMAN[0].1);
    assert_eq!(contents(&english, "XA.EXE"), ENGLISH[1].1);
}

#[test]
fn an_image_offers_only_the_renderers_on_it() {
    let player = Player::new("renderers");
    let german = player.disc("german", GERMAN).display().to_string();
    let english = player.disc("english", ENGLISH).display().to_string();
    assert!(!loader::renderer_available(&player.game, &german, GLIDE));
    assert!(loader::renderer_available(&player.game, &german, SOFTWARE));
    assert!(loader::renderer_available(&player.game, &english, GLIDE));
    assert!(loader::renderer_available(&player.game, &english, SOFTWARE));
}

#[test]
fn the_image_path_is_saved_and_used_next_launch() {
    let player = Player::new("saved");
    let german = player.disc("german", GERMAN);
    let mut settings = player.settings(Path::new(""));
    player.pick(&mut settings, &german);
    let runs = player.start(&settings);

    let again = player.next_launch();
    assert_eq!(Path::new(&again.folder), std::fs::canonicalize(&german).unwrap());
    assert_eq!(again.renderer, SOFTWARE);
    assert_eq!(player.start(&again), runs);
}

#[test]
fn switching_images_keeps_each_copy_and_its_saves() {
    let player = Player::new("switching");
    let (english, german) = (player.disc("english", ENGLISH), player.disc("german", GERMAN));
    let mut settings = player.settings(Path::new(""));
    player.pick(&mut settings, &german);
    let german_copy = player.start(&settings);
    std::fs::write(german_copy.join("SAVE"), "progress").unwrap();

    player.pick(&mut settings, &english);
    let english_copy = player.start(&settings);
    assert_ne!(english_copy, german_copy);
    assert_eq!(contents(&english_copy, "XA.EXE"), ENGLISH[1].1);

    player.pick(&mut settings, &german);
    assert_eq!(player.start(&settings), german_copy);
    assert_eq!(contents(&german_copy, "SAVE"), "progress");
}

#[test]
fn a_patched_copy_gives_its_image_glide() {
    let player = Player::new("patched");
    let german = player.disc("german", GERMAN);
    let mut settings = player.settings(Path::new(""));
    player.pick(&mut settings, &german);
    let copy = player.start(&settings);
    std::fs::write(copy.join("XA_3DFX.EXE"), GERMAN_GLIDE).unwrap();

    assert!(loader::renderer_available(&player.game, &settings.folder, GLIDE));
    assert!(!patch_wanted(&player.game, &settings.folder));
    assert_eq!(player.start(&settings), copy);
}

#[test]
fn every_way_of_picking_replaces_the_saved_folder() {
    let player = Player::new("picking");
    let old = player.beside.join("game");
    player.folder(&old, ENGLISH);
    let cue = player.disc("german", GERMAN);
    let installed = player.root.join("installed");
    player.folder(&installed, GERMAN);
    let cases = [
        (cue.clone(), cue.clone()),                                    // the sheet
        (cue.with_extension("bin"), cue.with_extension("bin")),        // the image
        (cue.parent().unwrap().to_path_buf(), cue.clone()),            // a folder holding only an image
        (installed.join("XA.EXE"), installed.clone()),                 // a file of the game
        (installed.clone(), installed.clone()),                        // the game's folder
    ];
    for (picked, chosen) in cases {
        let mut settings = player.settings(&old);
        assert!(player.pick(&mut settings, &picked).1, "{}", picked.display());
        assert_eq!(Path::new(&settings.folder), chosen);
        assert_eq!(contents(&player.start(&settings), "XA.EXE"), GERMAN[0].1, "{}", picked.display());
        assert_eq!(Path::new(&player.next_launch().folder), std::fs::canonicalize(&chosen).unwrap());
    }
}

#[test]
fn a_folder_in_the_way_is_left_alone() {
    let player = Player::new("in-the-way");
    player.folder(&player.beside.join("game"), ENGLISH);
    let blocked = player.beside.join("game-german-cd-1-0-1");
    player.folder(&blocked, &[("NOTES.TXT", "the player's")]);
    let mut settings = player.settings(Path::new(""));
    player.pick(&mut settings, &player.disc("german", GERMAN));

    assert!(prepare_start(&player.game, &settings, &mut |_, _| {}).unwrap_err().contains("in the way"));
    assert_eq!(contents(&blocked, "NOTES.TXT"), "the player's");
}

#[test]
fn an_image_of_something_else_is_refused() {
    let player = Player::new("other-disc");
    let mut settings = player.settings(Path::new(""));
    assert!(!player.pick(&mut settings, &player.disc("other", &[("OTHER.EXE", "other")])).1);
    assert!(prepare_start(&player.game, &settings, &mut |_, _| {}).is_err());
    assert!(!player.beside.join("game").exists());
}
