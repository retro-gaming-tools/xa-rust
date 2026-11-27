Extreme Assault Recompiled: Extreme Assault (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Extreme Assault**: the English CD (E 1.1.0), the German CD (R 1.0.1), or a copy patched to 1.23. No game data is included.

## Changes in 0.6-beta

- The game can play from a real CD or DVD drive: choose the drive (or a file on the disc) as the game.
- Disc images are recognised more reliably.
- *Skip launcher next time*: a new launcher row that starts the game straight away from then on. `--launcher`, or Shift held while the port starts, brings the launcher back.
- *Hotkeys...*: the port's own keys (fullscreen F11, frame counter F10) can be moved to other keys or turned off.
- *Texture filtering* on the advanced page: anisotropic filtering for Glide on the graphics card, 16x by default, for sharper ground at steep angles.
- The settings and the log are named after the program: `extreme-assault-rust.settings.ini` and `extreme-assault-rust.log`, so two ports in one folder keep their own. An existing `settings.ini` is still read.
- The log records the command line and the SHA-256 value of each game program the port checked.
- The README lists the programs' full SHA-256 values and how to check them, and says that the F keys the port keeps can't be used in the game.

The full history is in `CHANGELOG.md`.

## Downloads

- `extreme-assault-rust-0.6-beta-windows-x64.zip`: unzip anywhere and start the program. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `extreme-assault-rust-0.6-beta-linux-x64.AppImage`: make it executable and start it.
- `extreme-assault-rust-0.6-beta-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or game folder, or drop it onto the window.

## Features

- The 3Dfx Glide renderer, drawn by the port on the GPU (OpenGL) or the CPU, at higher resolutions than the original, with no 3Dfx card or Glide wrapper. The original software renderer is still available.
- The original start-up options in the launcher.

## Status

This is an early release. The German disc has no Glide program of its own: the launcher adds one from Blue Byte's German 3Dfx patch (1.10, `xa_r110.zip`, which you download yourself), on the row *Add the 3dfx patch*. If something doesn't work, please open an issue and attach `extreme-assault-rust.log` from beside the program.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/xa-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
