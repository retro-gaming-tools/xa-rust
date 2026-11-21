Extreme Assault Recompiled: Extreme Assault (Blue Byte, 1997) as a native Windows and Linux program, translated statically from the original into Rust, on a runtime built on SDL3. The coding was done by AI (Claude, by Anthropic). The testing was done by a human.

**You need your own copy of Extreme Assault**: the English CD (E 1.1.0), the German CD (R 1.0.1), or a copy patched to 1.23. No game data is included.

## Changes in 0.55-beta

- **Glide for the German CD (R 1.0.1).** Blue Byte's German 3Dfx patch (1.10, `xa_r110.zip`) uses a 16-bit DOS patcher that today's Windows can't run, so the port now unpacks it itself. Download `xa_r110.zip` from https://www.patches-scrolls.de/patch/1516/7/24201, choose the German disc in the launcher, and pick or drop the file on the new row *Add the 3dfx patch*. The port never downloads anything; it checks the patch and every file it writes by SHA-256, and switches the renderer to Glide. `--add-patch=xa_r110.zip` does the same without the launcher.
- The patch's Glide program runs at 60 frames a second, with its sound, like the other Glide versions.

## Downloads

- `extreme-assault-rust-0.55-beta-windows-x64.zip`: unzip anywhere and start the program. Windows may warn that the program is from an unknown publisher, because it isn't code-signed: choose "More info", then "Run anyway".
- `extreme-assault-rust-0.55-beta-linux-x64.AppImage`: make it executable and start it.
- `extreme-assault-rust-0.55-beta-SHA256SUMS.txt`: checksums of both.

In the launcher, pick your disc image or game folder, or drop it onto the window.

## Features

- The 3Dfx Glide renderer, drawn by the port on the GPU (OpenGL) or the CPU, at higher resolutions than the original, with no 3Dfx card or Glide wrapper. The original software renderer is still available.
- The original start-up options in the launcher.

## Status

This is an early release. The Glide programs run menus, missions, sound and CD music; the software programs are newer and less tried. The German disc has no Glide program of its own: new in 0.55-beta, the launcher adds one from Blue Byte's German 3Dfx patch (1.10, `xa_r110.zip`, which you download yourself), on the row *Add the 3dfx patch*. Please report bugs in the issues.

## Checking these files

GitHub built these files from the tagged source with `./build.sh`, in the workflow run linked to this release. Each file has a provenance attestation:

    gh attestation verify <file> --repo retro-gaming-tools/xa-rust

You can also build the release yourself with `./build.sh`.

## Legal

This is a fan project. It isn't affiliated with or endorsed by Ubisoft or Blue Byte. The recompiler, runtime, mods and build are MIT-licensed. The translated game code isn't licensed, because it belongs to the game's rights holders.
