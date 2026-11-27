//! Disc images, real CD and DVD drives and read-only game folders: the shared
//! `recomp-disc` crate (`tools/recomp/disc`), which every runtime uses, so a
//! fix there reaches every game. A game's own step, such as unpacking an
//! archive on its disc, goes through `unpack_with` and `copy_folder_with`.

pub use recomp_disc::*;
