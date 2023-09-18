use super::arch::hartid;
use crate::exit;
use crate::hsm::HsmCell;
use crate::hw_thread::Task;
use crate::init_heap;
use core::ptr::NonNull;
use fast_trap::{FastContext, FastResult, FlowContext, FreeTrapStack};

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

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}

extern "C" {
    static mut _sstack: u8;
    static _stack_size: usize;
    static _provide_base: usize;
}

pub struct Stack;
impl Stack {
    fn start(&self) -> usize {
        stack_start()
    }

    fn size(&self) -> usize {
        stack_size()
    }

    fn end(&self) -> usize {
        self.start() + self.size()
    }

    fn load_as_stack(&self) {
        // extern "C" {
        //     fn fast_handler(
        //         mut ctx: FastContext,
        //         a1: usize,
        //         a2: usize,
        //         a3: usize,
        //         a4: usize,
        //         a5: usize,
        //         a6: usize,
        //         a7: usize,
        //     ) -> FastResult;
        // }
        let context_ptr = unsafe { &mut CPU_CTXS[hartid()] }.context_ptr();
        core::mem::forget(
            FreeTrapStack::new(self.start()..self.end(), |_| {}, context_ptr, fast_handler)
                .unwrap()
                .load(),
        );
    }
}

#[no_mangle]
pub extern "C" fn stack_start() -> usize {
    let m_sstack = unsafe { &mut _sstack } as *mut _ as usize;
    #[cfg(feature = "stack_non_priv")]
    {
        m_sstack + stack_size() * hartid()
    }
    #[cfg(not(feature = "stack_non_priv"))]
    {
        m_sstack
    }
}

#[no_mangle]
pub extern "C" fn stack_size() -> usize {
    let m_stack_size = unsafe { &_stack_size } as *const usize as usize;
    let m_provide_base = unsafe { &_provide_base } as *const usize as usize;
    m_stack_size - m_provide_base
}

// #[export_name = "vfw_start"]
// fn vfw_start() {
//     use crate::hw_thread::idle;
//     extern "C" {
//         fn main() -> u32;
//     }
//     extern "C" {
//         fn __boot_core_init();
//     }
//     extern "C" {
//         fn __pre_init();
//     }
//     unsafe { __pre_init() };
//     let hartid = hartid();
//     if hartid == 0 {
//         init_bss();
//         init_heap();
//         unsafe { __boot_core_init() };
//         let ret = unsafe { main() };
//         exit(ret);
//     } else {
//         idle()
//     }
// }
use riscv::register::{
    mcause::{self, Exception as E, Trap as T},
    mepc, mtval, mtvec, satp, sstatus,
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
        Stack.load_as_stack();
        // unsafe { __boot_core_init() };
        // let ret = unsafe { main() };
        // exit(ret);
        unsafe { &mut CPU_CTXS[hartid] }.hsm.remote().start(Task {
            entry: __main as usize,
            args: [0; 8],
        });
    } else {
        Stack.load_as_stack();
        // idle()
    }
    unsafe {
        mtvec::write(fast_trap::trap_entry as usize, mtvec::TrapMode::Direct);
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
        CPU_CTXS[hartid()].hsm.local().stop();
        fast_trap::trap_entry();
    }
}

pub extern "C" fn fast_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    #[inline]
    fn boot(mut ctx: FastContext, hartid: usize, task: &Task) -> FastResult {
        unsafe {
            for i in 0..task.args.len() {
                CPU_CTXS[hartid].trap.a[i] = task.args[i];
            }
            CPU_CTXS[hartid].trap.pc = task.entry;
            CPU_CTXS[hartid].trap.sp = task.entry;
            CPU_CTXS[hartid].trap.ra = __done as usize;
            unsafe {
                core::arch::asm!("csrr {sp}, mscratch
                mv {gp}, gp
                ", 
                sp = out(reg) CPU_CTXS[hartid].trap.sp,
                gp = out(reg) CPU_CTXS[hartid].trap.gp,
                );
            }
            ctx.switch_to(CPU_CTXS[hartid].context_ptr())
        }
    }
    loop {
        match unsafe { &mut CPU_CTXS[hartid()].hsm.local().start() } {
            Ok(task) => {
                break boot(ctx, hartid(), &task);
            }
            Err(crate::hsm::HsmState::Stopped) => {
                crate::wait_ipi();
            }
            _ => {
                match mcause::read().cause() {
                    T::Exception(E::MachineEnvCall) => {
                        //fork on other core
                        let hart_target = a1;
                        let entry = a2;
                        let arg_len = a3;
                        let args = a4;
                        // crate::println!(
                        //     "hart_target = {:#x}, entry= {:#x}, arg_len = {:#x}",
                        //     hart_target,
                        //     entry,
                        //     arg_len
                        // );
                        let mut task = Task {
                            entry,
                            args: [0; 8],
                        };
                        for i in 0..arg_len {
                            unsafe {
                                task.args[i] =
                                    *((args + i * core::mem::size_of::<usize>()) as *const usize)
                            };
                        }
                        // crate::println!("begin fork");
                        crate::send_ipi(hart_target).unwrap();
                        let ret =
                            unsafe { CPU_CTXS[hart_target].hsm.remote().start(task) as usize };
                        ctx.regs().a[0] = ret;
                        mepc::write(mepc::read().wrapping_add(4));
                        break ctx.restore();
                    }
                    e => panic!(
                        "stopped with unsupported trap {:?}, mepc = {:#x}",
                        e,
                        mepc::read()
                    ),
                }
            }
        }
    }
}

pub fn new_try_fork_on(
    _call: usize,
    hart_target: usize,
    entry: usize,
    arg_len: usize,
    args: &[usize],
) -> bool {
    unsafe {
        let mut ret: usize = 0;
        core::arch::asm!("ecall", in("a1") hart_target, in("a2") entry, in("a3") arg_len, in("a4") args.as_ptr() as usize, out("a0") ret,clobber_abi("C"),);
        crate::println!("new_try_fork_on = {}", ret);
        ret != 0
    }
}
pub struct HartContext {
    trap: FlowContext,
    hsm: HsmCell<Task>,
}

impl HartContext {
    const fn new() -> Self {
        HartContext {
            trap: FlowContext::ZERO,
            hsm: HsmCell::new(),
        }
    }
    #[inline]
    fn init(&mut self) {
        self.hsm = HsmCell::new();
    }

    #[inline]
    fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

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

#[link_section = ".synced.bss"]
static mut CPU_CTXS: [HartContext; MAX_CORES] = [const { HartContext::new() }; MAX_CORES];
