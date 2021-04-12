#![no_std]
#![no_main]
#![feature(llvm_asm)]
extern crate compiler_builtins;
extern crate platform;
use platform::*;
#[export_name = "main"]
fn svcall_test() -> u32 {
    for _ in 0..5 {
        svcall!("sv_display2", "method2\0".as_ptr());
    }
    for i in 0..5 {
        svcall!("sv_display1", "method1\0".as_ptr(), i + 5);
    }
    1
}
