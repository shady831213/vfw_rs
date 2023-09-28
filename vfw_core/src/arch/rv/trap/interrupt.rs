use super::{default_trap_handler, dummy_trap_handler};
use crate::{per_cpu_offset, PER_CPU_LEN};

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

pub(super) const INT_VECTOR_LEN: usize = 12;

#[inline]
pub(super) fn interrupts() -> &'static mut [[InterruptVector; INT_VECTOR_LEN]] {
    crate::relocation!(mut __INTERRUPTS: [[InterruptVector; INT_VECTOR_LEN]; PER_CPU_LEN])
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
