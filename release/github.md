Extreme Assault Recompiled: Extreme Assault (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Extreme Assault**: the English CD (E 1.1.0), the German CD (R 1.0.1), or a copy patched to 1.23. No game data is included.

## Changes in 0.56-beta

- A chosen disc image starts its own version: the German disc could start an English game that was unpacked earlier. Each version keeps its own folder and saves.
- The launcher offers only the renderers the chosen disc has, and remembers the disc image's path.
- A folder that holds only a disc image can be chosen as the game.
- The software renderer works on a German copy with the 3Dfx patch. It stopped with error 71.
- `recomp.log` beside the program records the launcher's steps, the game's output and any crash, to attach to an issue. A message names it when the game stops with an error. The run before's is kept as `recomp-previous.log`.
- Fixed a memory bug that could cut data short when the game resized a block of memory.
- The README lists the supported versions and known limits in the same form as the other ports, and says that the original programs are DOS programs.
- The licence now excludes the translated game code in `programs/`. It named only `src/code/`, which this port doesn't use.

The full history is in `CHANGELOG.md`.

## Downloads

- `extreme-assault-rust-0.56-beta-windows-x64.zip`: unzip anywhere and start the program. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `extreme-assault-rust-0.56-beta-linux-x64.AppImage`: make it executable and start it.
- `extreme-assault-rust-0.56-beta-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or game folder, or drop it onto the window.

## Features

- The 3Dfx Glide renderer, drawn by the port on the GPU (OpenGL) or the CPU, at higher resolutions than the original, with no 3Dfx card or Glide wrapper. The original software renderer is still available.
- The original start-up options in the launcher.

## Status

This is an early release. The German disc has no Glide program of its own: the launcher adds one from Blue Byte's German 3Dfx patch (1.10, `xa_r110.zip`, which you download yourself), on the row *Add the 3dfx patch*. If something doesn't work, please open an issue and attach `recomp.log` from beside the program.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/xa-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
