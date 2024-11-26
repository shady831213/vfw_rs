#![no_std]
#![feature(alloc_error_handler)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(naked_functions)]
#![feature(linkage)]
extern crate alloc;
pub extern crate paste;
pub mod arch;
mod heap;
mod hw_thread;
pub use heap::*;
pub use hw_thread::*;
use vfw_primitives::*;
mod hsm;
pub use hsm::*;
mod msg;
pub use msg::*;
mod stack;
pub use stack::*;
mod platform;
mod rt;
pub use platform::*;
pub use rt::*;
mod trap;
pub use trap::*;
