## The launcher

![The launcher](release/launcher.png)

The launcher opens before the game. Your choices are saved in `settings.ini` beside the program (beside the `.AppImage` file on Linux), and `--play` on the command line skips the launcher.

- **Game files**: where your copy of the game is. Click *Choose folder...* or press Enter to pick a folder, or drop a folder or disc image onto the window. The line below it says which version was found, or what's missing.
- **Choose a disc image or game file**: pick a disc image or any file of the game.
- **3D resolution**: the size the Glide renderer draws the 3D view at, from the original 640 x 480 upwards. With the Software renderer it's greyed out.
- **Display**: *Window* or *Fullscreen*.
- **Renderer**: *Glide* (`XA_3DFX.EXE`, drawn by the port) or *Software* (`XA.EXE`, the game's own renderer). Renderers the chosen copy doesn't have are skipped, such as Glide on the German disc.
- **Advanced settings...** opens a second page, described below.
- **Start game** and **Quit**.

### Advanced settings

The defaults suit most machines. *Back* or Escape returns to the first page.

- **3D drawn by**: *GPU (OpenGL)* (the default) or *CPU*, for Glide.
- **Only 256-colour modes**, **Only low resolutions** and **Fixed texture memory**: the original's start-up options, described under *Supported versions*. Off by default.

The launcher works with the mouse, the keyboard (arrow keys to choose and change, Enter to confirm, Escape to quit) and a controller (d-pad to choose and change, A to confirm, Start to start the game, B to quit).

## Supported versions

The game came as two programs: `XA_3DFX.EXE` for 3Dfx (Glide) cards and `XA.EXE`, the software renderer. The port carries both, checks the one it starts by SHA-256, and lets you choose in the launcher (*Renderer*: Glide or Software). It supports:

| Version | Glide (`XA_3DFX.EXE`) | Software (`XA.EXE`) | Where it comes from |
|---|---|---|---|
| English CD, E 1.1.0 | `7abb3557…` | `746bb06b…` | The English disc, as a disc image (`.cue`/`.bin`) or its files |
| Patched 1.23 / 1.22 | `c24019df…` (1.23) | `952c865f…` (1.22) | An installed game with Blue Byte's updates applied |
| German CD, R 1.0.1 | none on the disc | `e69a0492…` | The German disc, as a disc image or its files |

State: the Glide programs run menus, missions, sound and CD music. The software programs reach the menus and missions (English 1.1 played mission 1, German 1.0.1 reaches the main menu); they're newer and less tried. The German disc has no Glide program, so the launcher uses software for it. The German 3Dfx patch (1.10, `xa_r110`) isn't supported yet.

- **Glide** is drawn by the port at the resolution you choose (*3D resolution*), on the graphics card or the processor.
- **Software** is the game's own renderer, at the game's own resolutions. Choose them in the game: *Options*, *Resolution Setup* lists all of its low- and high-resolution methods, in 256 colours or HiColor, as on a real VESA card.
- The original's start-up options are rows on the launcher's advanced page, since a modern machine doesn't need them: *Only 256-colour modes* (`forbid16bitcolor`, what `LOWCOL.BAT` did), *Only low resolutions* (`forbidhires`, what `LOWMEM.BAT` did) and *Fixed texture memory* (`forbidautoalloc`). They're off by default, as with `START.BAT`. Two options aren't offered: `nonetwork`, because the port has no IPX network to switch off, and `ff` for force-feedback joysticks, which the port doesn't drive.

### Files Glide needs

For Glide, the game folder needs `XA_3DFX.EXE` with one of the hashes in the table above. Nothing else from a 3Dfx patch is needed:

- **English CD 1.1.0:** everything is on the disc. Use the disc image or the disc's files as they are; no patch.
- **Patched 1.23:** use the folder Blue Byte's update left behind, with its `XA_3DFX.EXE` (`c24019df…`) and the `DATA` folder of the install. The update's `XA.EXE` (1.22) is the software program.
- **`GLIDE2X.OVL` isn't needed.** That was the driver for a real Voodoo card; the port draws Glide itself and never loads it. It can be missing, or be another Glide wrapper's file.
- **The 3Dfx cockpit pictures** (`DATA\GLOBAL\COCKPIT\3DFX\*.RLE`) come with the English disc and the install; keep them in place.
- **German CD 1.0.1:** the German 3Dfx patch (1.10, `xa_r110.zip`) isn't supported yet, so the German version runs in software only.

Other patch levels and other languages' discs aren't supported: their programs differ, and the launcher says which version it found.

With a disc image, the port unpacks the disc once and plays its CD music straight from the image. It also starts with CD music and the controller switched on; a patched install keeps its own settings.
