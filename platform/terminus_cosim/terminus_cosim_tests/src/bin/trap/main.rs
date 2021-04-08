#![no_std]
#![no_main]
#![feature(llvm_asm)]
extern crate platform;
extern crate compiler_builtins;
use platform::*;
use riscv::register::{mcause, mepc, mtvec::TrapMode};
#[export_name = "main"]
fn trap_test() -> u32 {
    unsafe { register_exception_handler(my_exp_handler) };
    init_trap(TrapMode::Vectored);
    unsafe {
        llvm_asm!("ecall"
            :
            :
            :);
    }
    0xff
}

#[no_mangle]
extern "C" fn my_exp_handler(trap_frame: &mut TrapFrame) {
    println!("mepc:{:#x?}", mepc::read());
    println!("exception cuase:{:#x?}", mcause::read().cause());
    println!("frame:{:#x?}", trap_frame);
    mepc::write(mepc::read().wrapping_add(4));
    println!("mepc:{:#x?}", mepc::read());
}
