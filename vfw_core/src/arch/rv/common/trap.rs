use crate::{per_cpu_offset, PER_CPU_LEN};
use riscv::register::{mcause, mepc, mtval};

#[cfg(target_pointer_width = "32")]
core::arch::global_asm!(include_str!("rv32.S"));

#[cfg(target_pointer_width = "64")]
core::arch::global_asm!(include_str!("rv64.S"));

core::arch::global_asm!(include_str!("trap.S"));

#[derive(Debug)]
#[repr(C)]
pub struct TrapFrame {
    pub zero: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

impl TrapFrame {
    pub fn update(&mut self, dst: u8, value: usize) {
        if dst > 31 {
            panic!("invalid target {}", dst)
        }
        unsafe {
            *((self as *mut TrapFrame as usize + (dst as usize) * core::mem::size_of::<usize>())
                as *mut usize) = value
        }
    }
    pub fn get(&self, src: u8) -> usize {
        if src > 31 {
            panic!("invalid src {}", src)
        }
        unsafe {
            *((self as *const TrapFrame as usize + (src as usize) * core::mem::size_of::<usize>())
                as *const usize)
        }
    }
}

#[repr(usize)]
pub enum Interrupt {
    UserSoft = 0,
    SupervisorSoft = 1,
    MachineSoft = 3,
    UserTimer = 4,
    SupervisorTimer = 5,
    MachineTimer = 7,
    UserExternal = 8,
    SupervisorExternal = 9,
    MachineExternal = 11,
}

pub union InterruptVector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

impl InterruptVector {
    pub unsafe fn handle(&self) {
        if self.reserved == 0 {
            default_trap_handler();
        } else {
            (self.handler)();
        }
    }

    pub unsafe fn handle_or_dummy(&self) {
        if self.reserved == 0 {
            dummy_trap_handler();
        } else {
            (self.handler)();
        }
    }
}

pub union ExceptionVector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

impl ExceptionVector {
    pub unsafe fn handle(&self) {
        if self.reserved == 0 {
            default_trap_handler();
        } else {
            (self.handler)();
        }
    }
}

pub fn default_trap_handler() {
    panic!(
        "Unexpected trap: cause:{:?}, mepc:{:x}, mtval:{:x}",
        mcause::read(),
        mepc::read(),
        mtval::read()
    );
}

pub fn dummy_trap_handler() {}

pub(crate) const INT_VECTOR_LEN: usize = 12;

#[no_mangle]
extern "C" fn start_trap_rust() {
    unsafe {
        let cause = mcause::read();
        if cause.is_exception() {
            expts()[per_cpu_offset()].handle();
        } else {
            let code = cause.code();
            if code < INT_VECTOR_LEN {
                let h = &interrupts()[per_cpu_offset()][code];
                h.handle();
            } else {
                default_trap_handler();
            }
        }
    }
}

#[inline]
pub(crate) fn expts() -> &'static mut [ExceptionVector] {
    crate::relocation!(mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN])
}

#[inline]
pub(crate) fn interrupts() -> &'static mut [[InterruptVector; INT_VECTOR_LEN]] {
    crate::relocation!(mut __INTERRUPTS: [[InterruptVector; INT_VECTOR_LEN]; PER_CPU_LEN])
}

pub unsafe fn register_exception_handler(f: unsafe extern "C" fn()) {
    expts()[per_cpu_offset()].handler = f;
}

pub unsafe fn register_int_handler(cause: Interrupt, f: unsafe extern "C" fn()) {
    let code = cause as usize;
    if code < INT_VECTOR_LEN {
        interrupts()[per_cpu_offset()][code].handler = f;
    }
}

#[link_section = ".cpu.bss"]
static mut __INTERRUPTS: [[InterruptVector; INT_VECTOR_LEN]; PER_CPU_LEN] =
    [const { [const { InterruptVector { reserved: 0 } }; INT_VECTOR_LEN] }; PER_CPU_LEN];

#[link_section = ".cpu.bss"]
static mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN] =
    [const { ExceptionVector { reserved: 0 } }; PER_CPU_LEN];
