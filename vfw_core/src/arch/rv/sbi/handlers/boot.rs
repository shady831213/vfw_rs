use crate::{
    arch::{trap_entry, FlowContext},
    cpu_ctx, hartid, Stack, Task, TaskId, TrapHandler, VfwStack,
};
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

fn prepare_for_trap(
    sbi_trap_handler: TrapHandler,
    hart_id: usize,
    start_addr: usize,
    opaque: usize,
) {
    unsafe { cpu_ctx(hartid()).hsm.local() }.stop();
    VfwStack.load_context(cpu_ctx(hartid()).context_ptr(), sbi_trap_handler);
    if hartid() == hart_id {
        cpu_ctx(hartid()).hsm.remote().start(Task {
            entry: start_addr,
            args: [opaque, 0, 0, 0, 0, 0, 0, 0],
            task_id: TaskId::new(hartid() as u16, 0),
        });
    }
}

pub fn to_next_stage(
    sbi_trap_handler: TrapHandler,
    hart_id: usize,
    start_addr: usize,
    opaque: usize,
) -> ! {
    prepare_for_trap(sbi_trap_handler, hart_id, start_addr, opaque);
    unsafe {
        trap_entry();
    }
    unreachable!();
}
