use super::arch::hartid;
use crate::exit;
use crate::init_heap;
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

#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_128"))]
pub const PER_CPU_LEN: usize = 128;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_64"))]
pub const PER_CPU_LEN: usize = 64;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_32"))]
pub const PER_CPU_LEN: usize = 32;
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
        feature = "max_cores_128",
        feature = "max_cores_64",
        feature = "max_cores_32",
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

#[export_name = "vfw_start"]
fn vfw_start() {
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
