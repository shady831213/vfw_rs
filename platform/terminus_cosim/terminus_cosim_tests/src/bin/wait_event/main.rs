#![no_std]
#![no_main]
#![feature(llvm_asm)]
extern crate platform;
use platform::*;
mod wait_event;
use wait_event::*;
#[export_name = "main"]
fn wait_event_test() -> u32 {
    for i in 0..10 {
        println!("get event {} resp {}!", i, mb_wait_event(i));
    }
    1
}
