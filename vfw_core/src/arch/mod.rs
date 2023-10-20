#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod rv;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub(crate) use rv::arch::*;
