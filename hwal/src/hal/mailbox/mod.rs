extern crate mailbox_rs;
use crate::rsrt::*;
use core::fmt;
pub use mailbox_rs::{mb_channel::*, mb_no_std::*, mb_rpcs::*};
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

#[cfg(feature = "mailbox_shared")]
static mut MB_SENDER: MBNbLockRefSender<MBChannel> =
    MBNbLockRefSender::new(unsafe { &mut MB_CH_RAW });

#[cfg(not(feature = "mailbox_shared"))]
static mut MB_SENDER: MBNbRefSender<MBChannel> = MBNbRefSender::new(unsafe { &mut MB_CH_RAW });

struct MBPrinter;

pub fn mb_sender() -> &'static mut impl MBNbSender {
    unsafe { &mut MB_SENDER }
}

impl fmt::Write for MBPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        mb_print(mb_sender(), s);
        Ok(())
    }
}

#[no_mangle]
extern "C" fn mailbox_cprint(
    fmt: *const u8,
    file: *const u8,
    line: u32,
    arg_len: u32,
    args: *const u32,
) {
    mb_cprint(mb_sender(), fmt, file, line, arg_len, args as *const MBPtrT)
}

#[macro_export]
macro_rules! cprint {
    ($fmt:expr) => {{
        extern "C" {
            fn mailbox_cprint(fmt: *const u8, file: *const u8, line: u32, arg_len: u32, args: *const u32);
        }
        let args:[u32;0] = [0;0];
        unsafe {
            mailbox_cprint(
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
            fn mailbox_cprint(fmt: *const u8, file: *const u8, line: u32, arg_len: u32, args: *const u32);
        }
        let args = [$($arg as u32,)*];
        unsafe {
            mailbox_cprint(
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

#[no_mangle]
extern "C" fn mailbox_svcall(method: *const u8, arg_len: u32, args: *const u32) -> u32 {
    mb_svcall(mb_sender(), method, arg_len, args as *const MBPtrT) as u32
}

#[macro_export]
macro_rules! svcall {
    ($method:expr) => {{
        extern "C" {
            fn mailbox_svcall(method: *const u8,
                arg_len: u32,
                args: *const u32) -> u32;
        }
        let args:[u32;0] = [0;0];
        unsafe {
            mailbox_svcall(
                core::concat!($method, "\0").as_bytes().as_ptr(),
                0,
                args.as_ptr(),
            )
        }
    }};
    ($method:expr, $($arg:expr),*) => {{
        extern "C" {
            fn mailbox_svcall(method: *const u8,
                arg_len: u32,
                args: *const u32) -> u32;
        }
        let args = [$($arg as u32,)*];
        unsafe {
            mailbox_svcall(
                core::concat!($method, "\0").as_bytes().as_ptr(),
                args.len() as u32,
                args.as_ptr(),
            )
        }
    }};
}

pub fn mailbox_print_str(s: &str) {
    use fmt::Write;
    MBPrinter.write_str(s).unwrap();
}

pub fn mailbox_exit(code: u32) -> ! {
    mb_exit(mb_sender(), code);
    loop {}
}

pub fn mailbox_memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    mb_memmove(mb_sender(), dest as MBPtrT, src as MBPtrT, n as MBPtrT);
    dest
}

pub fn mailbox_memset(dest: *mut u8, data: i32, n: usize) -> *mut u8 {
    mb_memset(mb_sender(), dest as MBPtrT, data as MBPtrT, n as MBPtrT);
    dest
}

pub fn mailbox_memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    mb_memcmp(mb_sender(), s1 as MBPtrT, s2 as MBPtrT, n as MBPtrT)
}

#[no_mangle]
extern "C" fn mailbox_fopen(path: *const u8, flags: u32) -> u32 {
    mb_fopen(mb_sender(), path as MBPtrT, flags)
}

#[no_mangle]
pub fn mailbox_fclose(fb: u32) {
    mb_fclose(mb_sender(), fb)
}

#[no_mangle]
extern "C" fn mailbox_fread(fb: u32, ptr: *mut u8, len: usize) -> usize {
    mb_fread(mb_sender(), fb, ptr as MBPtrT, len as MBPtrT)
}

#[no_mangle]
extern "C" fn mailbox_fwrite(fb: u32, ptr: *const u8, len: usize) -> usize {
    mb_fwrite(mb_sender(), fb, ptr as MBPtrT, len as MBPtrT)
}

#[no_mangle]
extern "C" fn mailbox_fseek(fb: u32, pos: MBPtrT) -> MBPtrT {
    mb_fseek(mb_sender(), fb, pos)
}

pub mod fs {
    extern crate alloc;
    use super::{
        mailbox_fclose, mailbox_fopen, mailbox_fread, mailbox_fseek, mailbox_fwrite, MBPtrT,
        MB_FILE_APPEND, MB_FILE_READ, MB_FILE_TRUNC, MB_FILE_WRITE,
    };
    use alloc::string::ToString;
    pub const HWAL_FILE_APPEND: u32 = MB_FILE_APPEND;
    pub const HWAL_FILE_READ: u32 = MB_FILE_READ;
    pub const HWAL_FILE_TRUNC: u32 = MB_FILE_TRUNC;
    pub const HWAL_FILE_WRITE: u32 = MB_FILE_WRITE;
    pub fn open(path: &str, flags: u32) -> File {
        File {
            fb: mailbox_fopen(
                format_args!("{}\0", path).to_string().as_bytes().as_ptr(),
                flags,
            ),
        }
    }
    pub struct File {
        fb: u32,
    }
    impl File {
        pub fn write(&mut self, buf: &[u8]) -> usize {
            mailbox_fwrite(self.fb, buf.as_ptr(), buf.len())
        }
        pub fn read(&mut self, buf: &mut [u8]) -> usize {
            mailbox_fread(self.fb, buf.as_mut_ptr(), buf.len())
        }
        pub fn seek(&mut self, pos: usize) -> usize {
            mailbox_fseek(self.fb, pos as MBPtrT) as usize
        }
    }
    impl Drop for File {
        fn drop(&mut self) {
            mailbox_fclose(self.fb)
        }
    }
}
