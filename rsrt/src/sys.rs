use crate::arch::*;
use crate::init_heap;
use alloc::string::ToString;
use core::alloc::Layout;
use core::fmt;
use core::panic::PanicInfo;
extern "C" {
    fn __exit(code: u32) -> !;
}

#[no_mangle]
pub extern "C" fn exit(code: u32) -> ! {
    unsafe { __exit(code) }
}

union PrintStrHandler {
    handler: fn(s: &str),
    reserved: usize,
}

#[link_section = ".synced.bss"]
static mut PRINT_STR: PrintStrHandler = PrintStrHandler { reserved: 0 };

pub fn init_print_str(f: fn(s: &str)) {
    unsafe {
        PRINT_STR.handler = f;
    }
}

pub fn __print_str(s: &str) {
    unsafe {
        if PRINT_STR.reserved == 0 {
            return;
        }
        (PRINT_STR.handler)(s)
    }
}
pub fn __print(args: fmt::Arguments) {
    __print_str(&args.to_string())
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => (crate::__print_str($fmt));
    ($fmt:expr, $($arg:tt)*) => ({
        crate::__print(core::format_args!($fmt, $($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (crate::print!(core::concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(core::concat!($fmt, "\n"), $($arg)*));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "full_panic")]
    println!("{}", info);
    #[cfg(not(feature = "full_panic"))]
    if let Some(location) = info.location() {
        println!(
            "panic in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    }
    exit(1)
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    crate::println!("oom!");
    crate::exit(2)
}

#[macro_export]
macro_rules! io_write8 {
    ($ptr:expr, $value:expr) => {
        unsafe { ($ptr as *mut u8).write_volatile($value as u8) }
    };
}

#[macro_export]
macro_rules! io_read8 {
    ($ptr:expr) => {
        unsafe { ($ptr as *const u8).read_volatile() }
    };
}

#[macro_export]
macro_rules! io_write16 {
    ($ptr:expr, $value:expr) => {
        unsafe { ($ptr as *mut u16).write_volatile($value as u16) }
    };
}

#[macro_export]
macro_rules! io_read16 {
    ($ptr:expr) => {
        unsafe { ($ptr as *const u16).read_volatile() }
    };
}

#[macro_export]
macro_rules! io_write32 {
    ($ptr:expr, $value:expr) => {
        unsafe { ($ptr as *mut u32).write_volatile($value as u32) }
    };
}

#[macro_export]
macro_rules! io_read32 {
    ($ptr:expr) => {
        unsafe { ($ptr as *const u32).read_volatile() }
    };
}

#[macro_export]
macro_rules! io_write64 {
    ($ptr:expr, $value:expr) => {
        unsafe { ($ptr as *mut u64).write_volatile($value as u64) }
    };
}

#[macro_export]
macro_rules! io_read64 {
    ($ptr:expr) => {
        unsafe { ($ptr as *const u64).read_volatile() }
    };
}

#[no_mangle]
pub extern "C" fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
    }
    (unsafe { &_num_cores }) as *const u8 as usize
}

fn init_bss() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        fn __init_bss(s: *mut u8, n: usize);
    }
    let m_sbss = unsafe { &mut _sbss } as *mut _ as usize;
    let m_ebss = unsafe { &mut _ebss } as *mut _ as usize;
    let size = m_ebss - m_sbss;
    if size > 0 {
        unsafe {
            __init_bss(m_sbss as *mut u8, size);
        }
    }
}

#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_16"))]
pub const PER_CPU_LEN: usize = 16;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_8"))]
pub const PER_CPU_LEN: usize = 8;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_4"))]
pub const PER_CPU_LEN: usize = 4;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_2"))]
pub const PER_CPU_LEN: usize = 2;
#[cfg(any(
    not(feature = "cpu_data_non_priv"),
    not(any(
        feature = "max_cores_16",
        feature = "max_cores_8",
        feature = "max_cores_4",
        feature = "max_cores_2"
    ))
))]
pub const PER_CPU_LEN: usize = 1;

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}

#[export_name = "rsrt_start"]
fn rsrt_start() {
    use crate::hw_thread::idle;
    extern "C" {
        fn main() -> u32;
    }
    extern "C" {
        fn __boot_core_init();
    }
    extern "C" {
        fn __pre_init();
    }
    unsafe { __pre_init() };
    let hartid = hartid();
    if hartid == 0 {
        init_bss();
        init_heap();
        unsafe { __boot_core_init() };
        let ret = unsafe { main() };
        exit(ret);
    } else {
        idle()
    }
}
