pub mod access;
pub mod handlers;
pub use rustsbi::*;

pub fn enter_next_privileged(
    privileged: riscv::register::mstatus::MPP,
    kernel_addr: usize,
    dtb_addr: usize,
) -> ! {
    unsafe {
        riscv::register::mepc::write(kernel_addr);
        riscv::register::mstatus::set_mpp(privileged);
        rustsbi::enter_privileged(riscv::register::mhartid::read(), dtb_addr)
    }
}
