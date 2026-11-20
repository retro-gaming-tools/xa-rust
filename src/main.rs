#![windows_subsystem = "windows"]

use recomp_runtime::{loader, GuestFn};

static MODULES: &[loader::Module] = &[
    loader::Module { file: "XA_3DFX.EXE", base: 0x1000000, sha256: "c24019dfd7641911cc090f58d0cd663772394d44cd770be4e5bed8969cc7e67b", program: true, name: "version 1.23 (patched)" },
    loader::Module { file: "XA_3DFX.EXE@1.1", base: 0x2000000, sha256: "7abb3557fa36c5425e6b6f5cf2d630609e3c4edce229cd20d84bd5bb7aa224bb", program: true, name: "English CD 1.1.0" },
    loader::Module { file: "XA.EXE", base: 0x3000000, sha256: "952c865f2fcbb0abf1f7ecdb3f9e0433f0db837cf92381c3e97bd5395eeb35fb", program: true, name: "version 1.22 (patched)" },
    loader::Module { file: "XA.EXE@1.1", base: 0x4000000, sha256: "746bb06b1a1ac1e7ea15a1b64ccdb469048ecfd6084d6cec810a25c139519a7c", program: true, name: "English CD 1.1.0" },
    loader::Module { file: "XA.EXE@de", base: 0x5000000, sha256: "e69a049251c8ade591adb5e5ee02fed8b05d3313ba86ab4e6cd02408fbefd725", program: true, name: "German CD 1.0.1" },
];

/// Each program's functions, from its own crate.
static TABLES: &[&[(u32, GuestFn)]] = &[
    extreme_assault_code_xa_3dfx::TABLE,
    extreme_assault_code_xa_3dfx_1_1::TABLE,
    extreme_assault_code_xa::TABLE,
    extreme_assault_code_xa_1_1::TABLE,
    extreme_assault_code_xa_de::TABLE,
];

static GAME: loader::Game = loader::Game {
    title: "Extreme Assault",
    version: include_str!("../VERSION").trim_ascii(),
    needs: "Extreme Assault by Blue Byte: the English CD (version E 1.1.0), the German CD (R 1.0.1), or a copy patched to version 1.23",
    command: "XA_3DFX.EXE",
    switches: &[("forbid16bitcolor", 0x0, "advanced: Only 256-colour modes"), ("forbidhires", 0x0, "advanced: Only low resolutions"), ("forbidautoalloc", 0x0, "advanced: Fixed texture memory")],
    renderers: &[("XA_3DFX.EXE", "Glide"), ("XA.EXE", "Software")],
    languages: &[],
    mods: &[],
    modules: MODULES,
    tables: TABLES,
    icon: include_bytes!("../icon.rgba"),
    music: &[],
    registry: &[],
    cd: "EXTREME",
    install: &["EXTREME/ROOT", "EXTREME"],
    cd_tracks: &[0, 77466, 91657, 104482, 115112, 129533, 143062, 156293, 169805, 186273, 205881, 222628, 238577, 246942, 254289, 255151, 263708, 264650, 280577, 282047, 296689],
};

fn main() {
    loader::run(&GAME);
}
