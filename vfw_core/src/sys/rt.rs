use super::arch::hartid;
use crate::exit;
use crate::hsm::HsmCell;
use crate::hw_thread::{Task, TaskId};
use crate::init_heap;
use crate::stack::*;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU16, Ordering};
use fast_trap::{FastContext, FastResult, FlowContext};
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

#[cfg(feature = "max_cores_2")]
const MAX_CORES: usize = 2;
#[cfg(feature = "max_cores_4")]
const MAX_CORES: usize = 4;
#[cfg(feature = "max_cores_8")]
const MAX_CORES: usize = 8;
#[cfg(feature = "max_cores_16")]
const MAX_CORES: usize = 16;
#[cfg(feature = "max_cores_32")]
const MAX_CORES: usize = 32;
#[cfg(feature = "max_cores_64")]
const MAX_CORES: usize = 64;
#[cfg(feature = "max_cores_128")]
const MAX_CORES: usize = 128;

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
const MAX_CORES: usize = 1;

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}

use riscv::register::{
    mcause::{self, Exception as E, Trap as T},
    mepc, mstatus,
};
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
        new_try_fork_on(0, __main as usize, 0, &[]);
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

fn __main() -> ! {
    extern "C" {
        fn main() -> u32;
    }
    let ret = unsafe { main() };
    exit(ret);
}

#[no_mangle]
fn __done() {
    unsafe {
        cpu_ctx(hartid()).hsm.local().stop();
        fast_trap::trap_entry();
    }
}

const VFW_CALL: usize = 10;
#[repr(usize)]
enum VfwCall {
    Fork = 0,
    Join,
}

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
            cpu_ctx(hartid).trap.ra = __done as usize;
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
            Err(state) => {
                let cause = mcause::read();
                match cause.cause() {
                    T::Exception(E::Unknown) if cause.bits() == VFW_CALL => {
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
                            "stopped with unsupported trap {:?}, mepc = {:#x}",
                            e,
                            mepc::read()
                        ),
                    },
                }
            }
        }
    }
}
fn vfw_call_handler(
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
    match VfwCall::try_from(ctx.a0()) {
        Ok(VfwCall::Fork) => fork_call(ctx, a1, a2, a3, a4, a5, hartid),
        Ok(VfwCall::Join) => join_call(ctx, a1),
        Err(e) => panic!("Invalid VfwCall {}", e),
    }
}
fn fork_call(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    hartid: usize,
) -> FastResult {
    //fork on other core
    let hart_target = a1;
    let task_id = a2 as u16;
    let entry = a3;
    let arg_len = a4;
    let args = a5;
    let mut task = Task {
        entry,
        args: [0; 8],
        task_id: TaskId::new(hartid as u16, task_id),
    };
    for i in 0..arg_len {
        unsafe { task.args[i] = *((args + i * core::mem::size_of::<usize>()) as *const usize) };
    }
    crate::send_ipi(hart_target).unwrap();
    let ret = cpu_ctx(hart_target).hsm.remote().start(task) as usize;
    ctx.regs().a[0] = ret as usize;
    ctx.restore()
}

fn join_call(ctx: FastContext, a1: usize) -> FastResult {
    #[inline]
    fn finished(issued: u16, retired: u16) -> bool {
        if (issued >> 15) != (retired >> 15) {
            retired <= issued
        } else {
            retired >= issued
        }
    }
    let id = TaskId::from_u32(a1 as u32);
    let cpu = cpu_ctx(id.hart_id() as usize);
    loop {
        if cpu.hsm.remote().get_status().expect("Invalid State!") == crate::hsm::HsmState::Stopped
            && finished(id.task_id(), cpu.current.task_id())
        {
            break;
        }
        core::hint::spin_loop();
    }
    ctx.restore()
}

//should be moved into arch
fn try_fork_on(
    hart_target: usize,
    task_id: u16,
    entry: usize,
    arg_len: usize,
    args: &[usize],
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
            in("a1") hart_target, in("a2") task_id as usize, in("a3") entry, in("a4") arg_len, in("a5") args.as_ptr() as usize,
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

pub fn new_try_fork_on(
    hart_target: usize,
    entry: usize,
    arg_len: usize,
    args: &[usize],
) -> Option<TaskId> {
    try_fork_on(
        hart_target,
        HW_TIDS.fetch_add(1, Ordering::SeqCst),
        entry,
        arg_len,
        args,
    )
}

pub fn new_fork_on(hart_target: usize, entry: usize, arg_len: usize, args: &[usize]) -> TaskId {
    let task_id = HW_TIDS.fetch_add(1, Ordering::SeqCst);
    loop {
        if let Some(id) = try_fork_on(hart_target, task_id, entry, arg_len, args) {
            break id;
        }
        core::hint::spin_loop();
    }
}

pub fn new_fork(entry: usize, arg_len: usize, args: &[usize]) -> TaskId {
    let task_id = HW_TIDS.fetch_add(1, Ordering::SeqCst);
    loop {
        for i in 1..crate::num_cores() {
            if let Some(id) = try_fork_on(i, task_id, entry, arg_len, args) {
                return id;
            }
        }
        core::hint::spin_loop();
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
pub struct HartContext {
    trap: FlowContext,
    hsm: HsmCell<Task>,
    current: TaskId,
}

impl HartContext {
    const fn new() -> Self {
        HartContext {
            trap: FlowContext::ZERO,
            hsm: HsmCell::new(),
            current: TaskId::new(0, 0),
        }
    }

    #[inline]
    fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

#[link_section = ".synced.bss"]
static mut CPU_CTXS: [HartContext; MAX_CORES] = [const { HartContext::new() }; MAX_CORES];
#[link_section = ".synced.bss"]
static HW_TIDS: AtomicU16 = AtomicU16::new(0);

pub fn cpu_ctx(hartid: usize) -> &'static mut HartContext {
    unsafe { &mut CPU_CTXS[hartid] }
}
