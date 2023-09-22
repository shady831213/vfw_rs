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

pub fn rv_wait_ipi() {
    use riscv::asm::wfi;
    use riscv::register::mie;
    unsafe {
        let sie = mie::read().msoft();
        mie::set_msoft();
        wfi();
        if !sie {
            mie::clear_msoft();
        }
    }
}

pub fn rv_hart_id() -> usize {
    riscv::register::mhartid::read()
}

pub fn rv_save_flag() -> usize {
    unsafe {
        let flag = mstatus::read().mie() as usize;
        mstatus::clear_mie();
        flag
    }
}

pub fn rv_restore_flag(flag: usize) {
    unsafe {
        if flag != 0 {
            mstatus::set_mie();
        }
    }
}

#[macro_export]
macro_rules! read_csr {
    ($csr_number:expr) => {
        {
            unsafe {
                let r: usize;
                core::arch::asm!("csrrs {0}, {1}, x0", out(reg) r, const $csr_number, options(pure, nomem, nostack));
                r
            }
        }
    }
}

#[macro_export]
macro_rules! write_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrw x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! set_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrs x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! clr_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrc x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! relocation {
    (mut $sym:ident:$t:ty) => {
        unsafe {
            #[cfg(all(feature="reloc", target_arch = "riscv64"))]
            {
                &mut *(crate::relocation!(@do_asm $sym) as *mut $t)
            }
            #[cfg(not(all(feature="reloc", target_arch = "riscv64")))]
            {
                &mut $sym
            }
        }
    };
    ($sym:ident:$t:ty) => {
        unsafe {
            #[cfg(all(feature="reloc", target_arch = "riscv64"))]
            {
                &const *(crate::relocation!(@do_asm $sym) as *const $t)
            }
            #[cfg(not(all(feature="reloc", target_arch = "riscv64")))]
            {
                &const $sym
            }
        }
    };
    (@do_asm $sym:ident) => {
        {
                let o: usize;
                core::arch::asm!(
                "
            .option push
            .option norelax
            la {o},{i}
            .option pop", i = sym $sym,
                o = out(reg) o,
                );
                o
        }
    };
}

#[no_mangle]
pub fn wait_mcycle(cnt: usize) {
    let cur = riscv::register::mcycle::read();
    while riscv::register::mcycle::read().wrapping_sub(cur) < cnt {}
}

#[no_mangle]
pub fn mcycle() -> usize {
    riscv::register::mcycle::read()
}

#[no_mangle]
pub fn wait_mcycle64(cnt: u64) {
    let cur = riscv::register::mcycle::read64();
    while riscv::register::mcycle::read64().wrapping_sub(cur) < cnt {}
}

#[no_mangle]
pub fn mcycle64() -> u64 {
    riscv::register::mcycle::read64()
}

#[cfg(not(feature = "max_cores_1"))]
use crate::hw_thread::Task;
#[cfg(not(feature = "max_cores_1"))]
pub fn run_task(task: &Task) {
    unsafe {
        core::arch::asm!(
        "jalr ra,{0}",in(reg) task.entry,
        in("a0") task.args[0],
        in("a1") task.args[1],
        in("a2") task.args[2],
        in("a3") task.args[3],
        in("a4") task.args[4],
        in("a5") task.args[5],
        in("a6") task.args[6],
        in("a7") task.args[7],
        clobber_abi("C"),)
    }
}

pub fn rv_init_fast_trap() {
    Stack.load_as_stack(fast_handler);
    unsafe {
        core::arch::asm!("
        mv {gp}, gp
        ", 
        gp = out(reg) cpu_ctx(hartid()).trap.gp,
        );
    }
}

const VFW_CALL: usize = 10;

pub extern "C" fn fast_handler(
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
                    crate::hsm::HsmState::Stopped => {
                        core::hint::spin_loop();
                    }
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
#[no_mangle]
extern "C" fn try_fork_on(
    hart_target: usize,
    task_id: u16,
    entry: usize,
    arg_len: usize,
    args: *const usize,
) -> usize {
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
        ret
    }
}

//should be moved into arch
pub fn new_join(id: TaskId) {
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
