extern crate mailbox_rs;
use crate::rsrt::*;
use core::fmt;
pub use mailbox_rs::{mb_channel::*, mb_no_std::*};
#[link_section = ".mailbox_queue"]
static mut MB_CH_RAW: MBChannel = MBChannel::const_init();

#[no_mangle]
extern "C" fn __mb_save_flag() -> u32 {
    save_flag() as u32
}
#[no_mangle]
extern "C" fn __mb_restore_flag(flag: u32) {
    restore_flag(flag as usize)
}

lazy_static::lazy_static! {
    pub static ref MB_SENDER: MBNbRefSender<MBChannel> = MBNbRefSender::new(unsafe { &mut MB_CH_RAW });
}

struct MBPrinter;

impl fmt::Write for MBPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        mb_print(&MB_SENDER, s);
        Ok(())
    }
}

#[export_name = "cprint"]
extern "C" fn mailbox_cprint(
    fmt: *const u8,
    file: *const u8,
    line: u32,
    arg_len: u32,
    args: *const u32,
) {
    mb_cprint(&MB_SENDER, fmt, file, line, arg_len, args)
}

#[macro_export]
macro_rules! cprint {
    ($fmt:expr) => {{
        extern "C" {
            fn cprint(fmt: *const u8, file: *const u8, line: u32, arg_len: u32, args: *const u32);
        }
        let args:[u32;0] = [0;0];
        unsafe {
            cprint(
                core::concat!($fmt, "\0").as_bytes().as_ptr(),
                core::concat!(file!(), "\0").as_bytes().as_ptr(),
                line!(),
                0,
                args.as_ptr(),
            );
        }
    }};
    ($fmt:expr, $($arg:expr),*) => {{
        extern "C" {
            fn cprint(fmt: *const u8, file: *const u8, line: u32, arg_len: u32, args: *const u32);
        }
        let args = [$($arg as u32,)*];
        unsafe {
            cprint(
                core::concat!($fmt, "\0").as_bytes().as_ptr(),
                core::concat!(file!(), "\0").as_bytes().as_ptr(),
                line!(),
                args.len() as u32,
                args.as_ptr(),
            );
        }
    }};
}

#[macro_export]
macro_rules! cprintln {
    ($fmt:expr) => {crate::cprint!(core::concat!($fmt, "\n"))};
    ($fmt:expr, $($arg:expr),*) => {crate::cprint!(core::concat!($fmt, "\n"), $($arg),*)};
}

pub fn mailbox_print_str(s: &str) {
    use fmt::Write;
    MBPrinter.write_str(s).unwrap();
}

pub fn mailbox_exit(code: u32) -> ! {
    mb_exit(&MB_SENDER, code);
    loop {}
}
