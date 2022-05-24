#![no_std]
#![feature(alloc_error_handler)]
#![allow(incomplete_features)]
#![feature(inline_const)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(generic_const_exprs)]
#![feature(linked_list_remove)]
#![feature(asm_const)]
pub extern crate embedded_hal;
pub extern crate nb;
extern crate rsrt;
pub mod arch;
pub mod hal;
pub mod io;
pub mod utils;
pub use rsrt::*;

//must be after mailbox init
#[export_name = "__init_bss"]
fn init_bss(s: *mut u8, n: usize) {
    #[cfg(feature = "mailbox_rs")]
    {
        extern "C" {
            fn mailbox_memset(dest: *mut u8, data: i32, n: usize) -> *mut u8;
        }
        mem_invalid(s as usize, n);
        unsafe {
            mailbox_memset(s, 0, n);
        }
    }
    #[cfg(not(feature = "mailbox_rs"))]
    {
        extern "C" {
            fn memset(dest: *mut u8, data: i32, n: usize) -> *mut u8;
        }
        unsafe {
            memset(s, 0, n);
        }
        mem_wb(s as usize, n);
    }
}
