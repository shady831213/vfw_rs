use crate::arch::rv::riscv;
#[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
use crate::cprintln;
use riscv::register::{mcause, mepc, mtval};

#[cfg(target_pointer_width = "32")]
core::arch::global_asm!(include_str!("rv32.S"));

core::arch::global_asm!(include_str!("trap.S"));

macro_rules! panic_msg {
    ($fmt:expr) => {{
        #[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
        cprintln!($fmt);
        #[cfg(not(feature = "full_panic"))]
        panic!();
        #[cfg(feature = "full_panic")]
        panic!($fmt);
    }};
    ($fmt:expr, $fmt_panic:expr, $($arg:expr),*) => {{
        #[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
        cprintln!($fmt, $($arg),*);
        #[cfg(not(feature = "full_panic"))]
        panic!();
        #[cfg(feature = "full_panic")]
        panic!($fmt_panic, $($arg),*);
    }};
}

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
            panic_msg!("invalid target %d", "invalid target {}", dst)
        }
        unsafe {
            *((self as *mut TrapFrame as usize + (dst as usize) * core::mem::size_of::<usize>()) as *mut usize) = value
        }
    }
    pub fn get(&self, src: u8) -> usize {
        if src > 31 {
            panic_msg!("invalid src %d", "invalid src {}", src)
        }
        unsafe {
            *((self as *const TrapFrame as usize + (src as usize) * core::mem::size_of::<usize>()) as *const usize)
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
}

pub union ExceptionVector {
    pub handler: unsafe extern "C" fn(trap_frame: &mut TrapFrame),
    pub reserved: usize,
}

impl ExceptionVector {
    pub unsafe fn handle(&self, trap_frame: &mut TrapFrame) {
        if self.reserved == 0 {
            default_trap_handler();
        } else {
            (self.handler)(trap_frame);
        }
    }
}

pub fn default_trap_handler() {
    panic_msg!("Unexpected trap: cause:%x, mepc:%x, mtval:%x", "Unexpected trap: cause:{:x}, mepc:{:x}, mtval:{:x}", mcause::read().bits(), mepc::read(), mtval::read());
}


#[no_mangle]
extern "C" fn start_trap_rust(trap_frame: &mut TrapFrame) {
    unsafe {
        let cause = mcause::read();
        if cause.is_exception() {
            __EXCEPTIONS.handle(trap_frame);
        } else {
            let code = cause.code();
            if code < __INTERRUPTS.len() {
                let h = &__INTERRUPTS[code];
                h.handle();
            } else {
                default_trap_handler();
            }
        }
    }
}

pub unsafe fn register_exception_handler(f: unsafe extern "C" fn(trap_frame: &mut TrapFrame)) {
    __EXCEPTIONS.handler = f;
}

pub unsafe fn register_int_handler(cause: Interrupt, f: unsafe extern "C" fn()) {
    let code = cause as usize;
    if code < __INTERRUPTS.len() {
        __INTERRUPTS[code].handler = f;
    }
}

#[link_section = ".cpu.bss"]
static mut __INTERRUPTS: [InterruptVector; 12] = [const { InterruptVector { reserved: 0 } }; 12];

#[link_section = ".cpu.bss"]
static mut __EXCEPTIONS: ExceptionVector = ExceptionVector { reserved: 0 };