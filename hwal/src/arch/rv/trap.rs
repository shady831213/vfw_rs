use crate::arch::rv::riscv;
use riscv::register::mcause;
#[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
use crate::cprintln;
#[derive(Debug)]
#[repr(C)]
pub struct TrapFrame {
    ra: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    s0: usize,
    s1: usize,
}

impl TrapFrame {
    pub fn update(&mut self, dst: u8, value: usize) {
        match dst {
            8 => self.s0 = value,
            9 => self.s1 = value,
            10 => self.a0 = value,
            11 => self.a1 = value,
            12 => self.a2 = value,
            13 => self.a3 = value,
            14 => self.a4 = value,
            15 => self.a5 = value,
            16 => self.a6 = value,
            17 => self.a7 = value,
            5 => self.t0 = value,
            6 => self.t1 = value,
            7 => self.t2 = value,
            28 => self.t3 = value,
            29 => self.t4 = value,
            30 => self.t5 = value,
            31 => self.t6 = value,
            _ => {
                #[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
                cprintln!("invalid target %d", &[dst as u32]);
                panic!("invalid target {}", dst)
            }
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
    #[cfg(all(not(feature = "full_panic"), feature = "mailbox_rs"))]
    cprintln!("Unexpected trap: cause:%x", &[mcause::read().bits() as u32]);
    panic!("Unexpected trap: cause:{:#x?}", mcause::read().cause());
}
