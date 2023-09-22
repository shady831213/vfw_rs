use crate::*;
use fast_trap::{FastContext, FastResult, FlowContext};
use riscv::register::{
    mcause::{self, Exception as E, Trap as T},
    mepc, mstatus,
};
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

// use riscv::register::mstatus;

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

#[export_name = "vfw_start"]
fn vfw_start() {
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
        Stack.load_as_stack(cpu_ctx(hartid).context_ptr(), fast_handler);
        unsafe {
            core::arch::asm!("
            mv {gp}, gp
            ", 
            gp = out(reg) cpu_ctx(hartid).trap.gp,
            );
        }
        new_try_fork_on(0, vfw_main as usize, 0, &[]);
    } else {
        Stack.load_as_stack(cpu_ctx(hartid).context_ptr(), fast_handler);
        unsafe {
            core::arch::asm!("
            mv {gp}, gp
            ", 
            gp = out(reg) cpu_ctx(hartid).trap.gp,
            );
        }
    }
    unsafe {
        fast_trap::trap_entry();
    }
}

const VFW_CALL: usize = 10;

impl core::convert::TryFrom<usize> for VfwCall {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VfwCall::Fork),
            1 => Ok(VfwCall::Join),
            v => Err(v),
        }
    }
}
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
                        crate::wait_ipi();
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
    ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    let hartid = hartid();
    let ret = match VfwCall::try_from(ctx.a0()) {
        Ok(VfwCall::Fork) => fork_call(ctx, a1, a2, a3, a4, a5, hartid),
        Ok(VfwCall::Join) => join_call(ctx, a1),
        Err(e) => panic!("Invalid VfwCall {:#x}", e),
    };
    unsafe {
        core::arch::asm!(
            "
                csrw mcause, zero
            "
        );
    }
    ret
}
// boot sp can not include handler call stack
#[inline(always)]
fn fork_call(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    hartid: usize,
) -> FastResult {
    ctx.regs().a[0] = __fork_call(a1, a2, a3, a4, a5, hartid) as usize;
    ctx.restore()
    // //fork on other core
    // let hart_target = a1;
    // let task_id = a2 as u16;
    // let entry = a3;
    // let arg_len = a4;
    // let args = a5;
    // let mut task = Task {
    //     entry,
    //     args: [0; 8],
    //     task_id: TaskId::new(hartid as u16, task_id),
    // };
    // for i in 0..arg_len {
    //     unsafe { task.args[i] = *((args + i * core::mem::size_of::<usize>()) as *const usize) };
    // }
    // crate::send_ipi(hart_target).unwrap();
    // let ret = cpu_ctx(hart_target).hsm.remote().start(task) as usize;
    // ctx.regs().a[0] = ret as usize;
    // ctx.restore()
}
// boot sp can not include handler call stack
#[inline(always)]
fn join_call(ctx: FastContext, a1: usize) -> FastResult {
    __join_call(a1);
    // #[inline]
    // fn finished(issued: u16, retired: u16) -> bool {
    //     if (issued >> 15) != (retired >> 15) {
    //         retired <= issued
    //     } else {
    //         retired >= issued
    //     }
    // }
    // let id = TaskId::from_u32(a1 as u32);
    // let cpu = cpu_ctx(id.hart_id() as usize);
    // loop {
    //     if cpu.hsm.remote().get_status().expect("Invalid State!") == crate::hsm::HsmState::Stopped
    //         && finished(id.task_id(), cpu.current.task_id())
    //     {
    //         break;
    //     }
    //     core::hint::spin_loop();
    // }
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
