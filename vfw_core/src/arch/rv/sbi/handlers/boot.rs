use crate::{arch::FlowContext, hartid};
use riscv::register::{satp, sstatus};

#[inline]
pub fn sbi_boot(regs: &mut FlowContext, start_addr: usize, opaque: usize) {
    unsafe {
        sstatus::clear_sie();
        satp::write(0);
    }
    regs.a[0] = hartid();
    regs.a[1] = opaque;
    regs.pc = start_addr;
}
