Extreme Assault Recompiled: Extreme Assault (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Extreme Assault**: the English CD (E 1.1.0), the German CD (R 1.0.1), or a copy patched to 1.23. No game data is included.

## Downloads

- `extreme-assault-rust-<version>-windows-x64.zip`: unzip anywhere and start the program. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `extreme-assault-rust-<version>-linux-x64.AppImage`: make it executable and start it.
- `extreme-assault-rust-<version>-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or game folder, or drop it onto the window.

## Features

- The 3Dfx Glide renderer, drawn by the port on the GPU (OpenGL) or the CPU, at higher resolutions than the original, with no 3Dfx card or Glide wrapper. The original software renderer is still available.
- The original start-up options in the launcher.

## Status

This is an early release. The Glide programs run menus, missions, sound and CD music; the software programs are newer and less tried. The German disc has no Glide program, and the German 3Dfx patch (1.10) isn't supported yet. Please report bugs in the issues.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/xa-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
