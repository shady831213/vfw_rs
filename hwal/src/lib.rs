#![no_std]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![allow(incomplete_features)]
#![feature(inline_const)]
extern crate rsrt;
pub mod arch;
pub mod hal;
pub use rsrt::*;
