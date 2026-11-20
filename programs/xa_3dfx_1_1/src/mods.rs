//! Written by the recompiler: the hand-written files in `mods/` for this program.
#![allow(warnings)]

#[path = "../../../mods/glide.rs"]
pub mod glide;
pub use glide::*;
#[path = "../../../mods/miles.rs"]
pub mod miles;
pub use miles::*;
