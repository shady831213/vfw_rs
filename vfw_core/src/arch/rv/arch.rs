use crate::*;
use fast_trap::{FastContext, FastResult};
use riscv::register::{
    mcause::{self, Exception as E, Trap as T},
    mepc, mstatus,
};

#[cfg(not(feature = "stack_non_priv"))]
core::arch::global_asm!(include_str!("crt_priv_stack.S"));
#[cfg(feature = "stack_non_priv")]
core::arch::global_asm!(include_str!("crt_non_priv_stack.S"));

#[naked]
#[no_mangle]
pub(crate) unsafe extern "C" fn reuse_stack_for_trap() {
    core::arch::asm!(
        "   call t1, {move_stack}
            ret
        ",
        move_stack          =   sym fast_trap::reuse_stack_for_trap,
        options(noreturn),
    )
}

pub(crate) fn hartid() -> usize {
    riscv::register::mhartid::read()
}

pub(crate) fn save_flag() -> usize {
    unsafe {
        let flag = mstatus::read().mie() as usize;
        mstatus::clear_mie();
        flag
    }
}

pub(crate) fn restore_flag(flag: usize) {
    unsafe {
        if flag != 0 {
            mstatus::set_mie();
        }
    }
}

pub(crate) fn init_fast_trap() {
    unsafe {
        core::arch::asm!("
        mv {gp}, gp
        ", 
        gp = out(reg) cpu_ctx(hartid()).trap.gp,
        );
    }
}

const VFW_CALL: usize = 10;

pub(crate) extern "C" fn vfw_fast_handler(
    ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    #[inline]
    fn boot(ctx: FastContext, hartid: usize, task: &Task) -> FastResult {
        unsafe {
            mstatus::set_mpp(mstatus::MPP::Machine);
            for i in 0..task.args.len() {
                cpu_ctx(hartid).trap.a[i] = task.args[i];
            }
            cpu_ctx(hartid).trap.pc = task.entry;
            cpu_ctx(hartid).trap.ra = vfw_done as usize;
            // ------------------
            // | app stack      |  -
            // ------------------   |
            // | fast stack     |   | - stack
            // ------------------   |
            // | handler struct |  -

            // this sp including handler sturct + fast stack automatically
            core::arch::asm!("
                mv {sp}, sp
                ",
            sp = out(reg) cpu_ctx(hartid).trap.sp,
            );
            cpu_ctx(hartid).current = task.task_id;
            ctx.switch_to(cpu_ctx(hartid).context_ptr())
        }
    }
    loop {
        match unsafe { &mut cpu_ctx(hartid()).hsm.local().start() } {
            Ok(task) => {
                break boot(ctx, hartid(), &task);
            }
            Err(state) => match mcause::read().cause() {
                T::Exception(E::Unknown) if mcause::read().bits() == VFW_CALL => {
                    unsafe {
                        mstatus::set_mpp(mstatus::MPP::Machine);
                    }
                    break vfw_call_handler(ctx, a1, a2, a3, a4, a5, a6, a7);
                }
                e => match state {
                    crate::hsm::HsmState::Stopped => vfw_idle(),
                    _ => panic!(
                        "stopped with unsupported trap {:?}, mepc = {:#x}, state = {:?}",
                        e,
                        mepc::read(),
                        state
                    ),
                },
            },
        }
    }
}
// boot sp can not include handler call stack
#[inline(always)]
pub(crate) fn vfw_call_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    // let hartid = hartid();
    let mut a: [usize; 8] = [ctx.a0(), a1, a2, a3, a4, a5, a6, a7];
    let ret_len = vfw_call(&mut a);
    for i in 0..ret_len {
        ctx.regs().a[i] = a[i];
    }
    unsafe {
        core::arch::asm!(
            "
                csrw mcause, zero
            "
        );
    }
    ctx.restore()
}

//should be moved into arch
#[inline]
pub(crate) fn _try_fork_on(
    hart_target: usize,
    task_id: u16,
    entry: usize,
    arg_len: usize,
    args: *const usize,
) -> Option<TaskId> {
    unsafe {
        let mut ret: usize = VfwCall::Fork as usize;
        core::arch::asm!(
            "   la   t0,    1f
                csrw mepc,   t0
                csrw mcause, {cause}
                j    {trap}
             1:
            ",
            out("t0") _,
            inout("a0") ret,
            in("a1") hart_target, in("a2") task_id as usize, in("a3") entry, in("a4") arg_len, in("a5") args,
            cause = in(reg) VFW_CALL,
            trap  = sym fast_trap::trap_entry,
            clobber_abi("C"),
        );
        if ret == 0 {
            None
        } else {
            Some(TaskId::new(hart_target as u16, task_id))
        }
    }
}

//should be moved into arch
#[inline]
pub(crate) fn _join(id: TaskId) {
    unsafe {
        core::arch::asm!(
            "   la   t0,    1f
                csrw mepc,   t0
                csrw mcause, {cause}
                j    {trap}
             1:
            ",
            out("t0") _,
            in("a0") VfwCall::Join as usize,
            in("a1") id.raw() as usize,
            cause = in(reg) VFW_CALL,
            trap  = sym fast_trap::trap_entry,
            clobber_abi("C"),
        );
    }
}
