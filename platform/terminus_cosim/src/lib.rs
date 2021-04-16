#![no_std]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![allow(incomplete_features)]
#![feature(inline_const)]
extern crate hwal;
pub use hwal::arch::rv::riscv;
use hwal::arch::rv::rtapi::*;
pub use hwal::arch::rv::standard::{init_trap, register_exception_handler, register_int_handler};
pub use hwal::arch::rv::trap::*;
use hwal::hal::clint::Clint;
pub use hwal::hal::mailbox::fs;
use hwal::hal::mailbox::*;
pub use hwal::*;

#[export_name = "__arch_hart_id"]
fn hart_id() -> usize {
    rv_hart_id()
}

#[export_name = "__arch_save_flag"]
fn save_flag() -> usize {
    rv_save_flag()
}

#[export_name = "__arch_restore_flag"]
fn restore_flag(flag: usize) {
    rv_restore_flag(flag)
}

include!(concat!(env!("OUT_DIR"), "/rsrt_bindings.rs"));
static CLINT: Clint = Clint::new(CLINT_BASE as usize);

#[export_name = "__wait_ipi"]
fn wait_ipi() {
    rv_wait_ipi()
}

#[export_name = "__send_ipi"]
fn clint_send_soft(hart_id: usize) {
    CLINT.send_soft(hart_id);
}
#[export_name = "__clear_ipi"]
fn clint_clear_soft(hart_id: usize) {
    CLINT.clear_soft(hart_id);
}

#[export_name = "__exit"]
fn exit(code: u32) -> ! {
    mailbox_exit(code)
}

#[export_name = "__pre_init"]
fn pre_init() {}

#[export_name = "__boot_core_init"]
fn boot_core_init() {
    init_print_str(mailbox_print_str);
    set_arch_task_run(run_task);
}

#[no_mangle]
#[cfg(feature = "mailbox_mem")]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    mailbox_memmove(dest, src, n)
}

#[no_mangle]
#[cfg(feature = "mailbox_mem")]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    mailbox_memmove(dest, src, n)
}

#[no_mangle]
#[cfg(feature = "mailbox_mem")]
pub unsafe extern "C" fn memset(dest: *mut u8, data: i32, n: usize) -> *mut u8 {
    mailbox_memset(dest, data, n)
}

#[no_mangle]
#[cfg(feature = "mailbox_mem")]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    mailbox_memcmp(s1, s2, n)
}
