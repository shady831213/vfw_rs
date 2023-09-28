use crate::*;
use fast_trap::{EntireContext, EntireResult, FastContext, FastResult, FlowContext};
use paste::paste;
use riscv::register::{
    mcause::{self, Exception as E, Interrupt, Trap as T},
    mepc, mstatus,
};

macro_rules! exchange {
    () => {
        exchange!(sp)
    };

    ($reg:ident) => {
        concat!("csrrw ", stringify!($reg), ", mscratch, ", stringify!($reg))
    };
}

#[cfg(target_pointer_width = "32")]
#[macro_use]
mod arch {
    macro_rules! save {
        ($reg:ident => $ptr:ident[$pos:expr]) => {
            concat!(
                "sw ",
                stringify!($reg),
                ", 4*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }

    macro_rules! push_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "-4*", $size)
        };
    }

    macro_rules! pop_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "4*", $size)
        };
    }

    macro_rules! load {
        ($ptr:ident[$pos:expr] => $reg:ident) => {
            concat!(
                "lw ",
                stringify!($reg),
                ", 4*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }
}
#[cfg(target_pointer_width = "64")]
#[macro_use]
mod arch {
    macro_rules! save {
        ($reg:ident => $ptr:ident[$pos:expr]) => {
            concat!(
                "sd ",
                stringify!($reg),
                ", 8*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }

    macro_rules! push_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "-8*", $size)
        };
    }

    macro_rules! pop_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "8*", $size)
        };
    }

    macro_rules! load {
        ($ptr:ident[$pos:expr] => $reg:ident) => {
            concat!(
                "ld ",
                stringify!($reg),
                ", 8*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }
}

macro_rules! on_vfw_stack {
    ($name:ident, $entry: ident) => {
        paste! {
            #[naked]
            unsafe extern "C" fn [<$name _on_vfw_stack>]() {
                core::arch::asm!(
                    exchange!(),
                    push_stack!(1),
                    save!(ra => sp[0]),
                    "call {handler}",
                    load!(sp[0] => ra),
                    pop_stack!(1),
                    exchange!(),
                    "ret",
                    handler    = sym $entry,
                    options(noreturn),
                )
            }
        }
    };
}

const VFW_CALL: usize = 10;

#[inline(always)]
unsafe fn to_other_ctx(ctx: &FlowContext) {
    core::arch::asm!(
        "   mv         gp, {gp}
            mv         tp, {tp}
            csrw mscratch, sp
            csrw     mepc, {pc}
        ",
        gp = in(reg) ctx.gp,
        tp = in(reg) ctx.tp,
        pc = in(reg) ctx.pc,
    );
}

//only for machine level vfw run-time
//if need switch to other context, such as SBI, Stack.load_as_stack can be used to set other context and fast_handler
//for sbi, all machine level run time is in the trap scope, thus, all stack is available for fast_trap
//so vfw_fast_handler only handle machine level app
#[inline(always)]
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
    #[inline(always)]
    fn boot(mut ctx: FastContext, hartid: usize, task: &Task) -> FastResult {
        unsafe {
            mstatus::set_mpp(mstatus::MPP::Machine);
            cpu_ctx(hartid).current = task.task_id;

            let regs = ctx.regs();
            for i in 0..task.args.len() {
                regs.a[i] = task.args[i];
            }
            regs.pc = task.entry;
            regs.ra = vfw_done as usize;
            // ------------------
            // | app stack      |  -
            // ------------------   |
            // | fast stack     |   | - stack
            // ------------------   |
            // | handler struct |  -

            // this sp including handler sturct + fast stack automatically
            to_other_ctx(regs);
            FastResult::Restore
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
                T::Exception(_) => match state {
                    crate::hsm::HsmState::Stopped => vfw_idle(),
                    _ => {
                        break vfw_exception_handler(ctx, a1, a2, a3, a4, a5, a6, a7);
                    }
                },
                T::Interrupt(_) => break ctx.continue_with(vfw_interrupt_handler, ()),
            },
        }
    }
}

#[inline(always)]
pub(crate) extern "C" fn vfw_exception_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    unsafe {
        core::arch::asm!(
            "   mv         a0, {ctx}
        mv a1, {a1}
        mv a2, {a2}
        mv a3, {a3}
        mv a4, {a4}
        mv a5, {a5}
        mv a6, {a6}
        mv a7, {a7}
        ",
            ctx = in(reg) ctx.regs(),
            a1 = in(reg) a1,
            a2 = in(reg) a2,
            a3 = in(reg) a3,
            a4 = in(reg) a4,
            a5 = in(reg) a5,
            a6 = in(reg) a6,
            a7 = in(reg) a7,
            options(nomem, nostack),
        );
        exception_on_vfw_stack();
    };
    ctx.restore()
}

on_vfw_stack!(exception, exception_handler);

#[inline(always)]
unsafe extern "C" fn exception_handler(
    ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) {
    super::super::standard::trap::expts()[per_cpu_offset()].handle();
}

#[inline(always)]
pub(crate) extern "C" fn vfw_interrupt_handler(_ctx: EntireContext<()>) -> EntireResult {
    unsafe { interrupt_on_vfw_stack() };
    EntireResult::Restore
}

on_vfw_stack!(interrupt, interrupt_handler);

#[inline(always)]
unsafe extern "C" fn interrupt_handler(_ctx: EntireContext<()>) {
    let code = mcause::read().code();
    if code < super::super::standard::trap::INT_VECTOR_LEN {
        let h = &super::super::standard::trap::interrupts()[per_cpu_offset()][code];
        if Interrupt::from(code) == Interrupt::MachineTimer {
            h.handle_or_dummy();
        } else {
            h.handle();
        }
    } else {
        super::super::standard::trap::default_trap_handler();
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
