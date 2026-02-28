#![no_std]
#![feature(alloc_error_handler)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
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
mod init;
use init::*;

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

#[cfg(feature = "max_cores_2")]
pub const MAX_CORES: usize = 2;
#[cfg(feature = "max_cores_4")]
pub const MAX_CORES: usize = 4;
#[cfg(feature = "max_cores_8")]
pub const MAX_CORES: usize = 8;
#[cfg(feature = "max_cores_16")]
pub const MAX_CORES: usize = 16;
#[cfg(feature = "max_cores_32")]
pub const MAX_CORES: usize = 32;
#[cfg(feature = "max_cores_64")]
pub const MAX_CORES: usize = 64;
#[cfg(feature = "max_cores_128")]
pub const MAX_CORES: usize = 128;

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub const MAX_CORES: usize = 1;

#[no_mangle]
pub extern "C" fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
    }
    let m_num_cores = &raw const _num_cores as *const u8 as usize;
    m_num_cores
}

#[no_mangle]
pub extern "C" fn hartid() -> usize {
    arch::hartid()
}

#[no_mangle]
pub extern "C" fn save_flag() -> usize {
    arch::save_flag()
}

#[no_mangle]
pub extern "C" fn restore_flag(flag: usize) {
    arch::restore_flag(flag)
}
