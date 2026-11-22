//! The runtime that recompiled games run on. This is original code: it holds
//! nothing from any game. Generated code lives in the game's `-rust` repo.

// The runtime's own `println!` and `print!` also go into `recomp.log`; these
// replace the standard ones in every module below.
macro_rules! println {
    () => {{ ::std::println!(); $crate::runlog::write("\n"); }};
    ($($argument:tt)*) => {{
        let text = ::std::format!($($argument)*);
        ::std::println!("{text}");
        $crate::runlog::line(&text);
    }};
}

macro_rules! print {
    ($($argument:tt)*) => {{
        let text = ::std::format!($($argument)*);
        ::std::print!("{text}");
        $crate::runlog::write(&text);
    }};
}

pub mod runlog;
pub mod cpu;
pub mod host;
pub mod loader;
pub mod le;
pub mod pe;
pub mod sys;

pub use cpu::{Cpu, GuestFn};
