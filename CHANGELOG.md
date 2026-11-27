# Changelog

Newest first. Each version has a tag `v<version>`, and `./changes.sh <old> <new>` shows the full diff between two of them.

## 0.6-beta (2026-11-27)

- The game can play from a real CD or DVD drive: choose the drive (or a file on the disc) as the game.
- Disc images are recognised more reliably.
- *Skip launcher next time*: a new launcher row that starts the game straight away from then on. `--launcher`, or Shift held while the port starts, brings the launcher back.
- *Hotkeys...*: the port's own keys (fullscreen F11, frame counter F10) can be moved to other keys or turned off.
- *Texture filtering* on the advanced page: anisotropic filtering for Glide on the graphics card, 16x by default, for sharper ground at steep angles.
- The settings and the log are named after the program: `extreme-assault-rust.settings.ini` and `extreme-assault-rust.log`, so two ports in one folder keep their own. An existing `settings.ini` is still read.
- The log records the command line and the SHA-256 value of each game program the port checked.
- The README lists the programs' full SHA-256 values and how to check them, and says that the F keys the port keeps can't be used in the game.

## 0.56-beta (2026-11-22)

- A chosen disc image starts its own version: the German disc could start an English game that was unpacked earlier. Each version keeps its own folder and saves.
- The launcher offers only the renderers the chosen disc has, and remembers the disc image's path.
- A folder that holds only a disc image can be chosen as the game.
- The software renderer works on a German copy with the 3Dfx patch. It stopped with error 71.
- `recomp.log` beside the program records the launcher's steps, the game's output and any crash, to attach to an issue. A message names it when the game stops with an error. The run before's is kept as `recomp-previous.log`.
- Fixed a memory bug that could cut data short when the game resized a block of memory.
- The README lists the supported versions and known limits in the same form as the other ports, and says that the original programs are DOS programs.
- The licence now excludes the translated game code in `programs/`. It named only `src/code/`, which this port doesn't use.

## 0.55-beta (2026-11-21)

- **Glide for the German CD (R 1.0.1).** The German disc came with the software renderer only, and Blue Byte's German 3Dfx patch (1.10, `xa_r110.zip`) uses a 16-bit DOS patcher that today's Windows can't run. The port now unpacks the patch itself: download `xa_r110.zip` from https://www.patches-scrolls.de/patch/1516/7/24201, choose the German disc in the launcher, and pick or drop the file on the new row *Add the 3dfx patch*. The port never downloads anything. It checks the patch and every file it writes by SHA-256, unpacks a disc image first, and switches the renderer to Glide. `--add-patch=xa_r110.zip` does the same without the launcher.
- The patch's Glide program (`XA_3DFX.EXE` R 1.1.0, `24f23a78…`) is recompiled with the others, with its sound, and runs at 60 frames a second like the other Glide versions.
- The patch format (Pocket Soft's .RTPatch) is read as worked out by [rtptool](https://github.com/bwrsandman/rtptool) (MIT licence).

## 0.5-beta (2026-11-20)

The first public release.
