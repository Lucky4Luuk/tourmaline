#![no_std]

#[macro_use] extern crate alloc;

use lol_alloc::{FreeListAllocator, LockedAllocator};

#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

// Expose the raw ABI
pub mod abi;

pub mod string { pub use alloc::string::String; }
pub mod vec { pub use alloc::vec::Vec; }
// pub mod collections { pub use alloc::collections::VecDeque; } // Seems to cause heap allocation errors
pub mod fmt { pub use core::fmt::*; }
pub mod io;

pub fn kernel_log<S: Into<string::String>>(s: S) {
    let s = s.into();
    abi::abi_sys_log(s);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
