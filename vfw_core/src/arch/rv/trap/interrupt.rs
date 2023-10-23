use crate::arch::FlowContext;
use crate::arch::{default_trap_handler, dummy_trap_handler};
use crate::{per_cpu_offset, PER_CPU_LEN};
use riscv::register::mcause;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
impl core::convert::From<mcause::Interrupt> for Interrupt {
    #[inline]
    fn from(cause: mcause::Interrupt) -> Self {
        match cause {
            mcause::Interrupt::UserSoft => Interrupt::UserSoft,
            mcause::Interrupt::SupervisorSoft => Interrupt::SupervisorSoft,
            mcause::Interrupt::MachineSoft => Interrupt::MachineSoft,
            mcause::Interrupt::UserTimer => Interrupt::UserTimer,
            mcause::Interrupt::SupervisorTimer => Interrupt::SupervisorTimer,
            mcause::Interrupt::MachineTimer => Interrupt::MachineTimer,
            mcause::Interrupt::UserExternal => Interrupt::UserExternal,
            mcause::Interrupt::SupervisorExternal => Interrupt::SupervisorExternal,
            mcause::Interrupt::MachineExternal => Interrupt::MachineExternal,
            mcause::Interrupt::Unknown => panic!("unknown interrupt cause!"),
        }
    }
}

pub union InterruptVector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

impl InterruptVector {
    pub unsafe fn handle(&self, ctx: &mut FlowContext) {
        if self.reserved == 0 {
            default_trap_handler(ctx);
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

const INT_VECTOR_LEN: usize = 12;

#[inline(always)]
pub fn interrupt_handler(ctx: &mut FlowContext, cause: Interrupt) {
    unsafe {
        if (cause as usize) < INT_VECTOR_LEN {
            let h = &interrupts()[per_cpu_offset()][cause as usize];
            h.handle(ctx);
        } else {
            default_trap_handler(ctx);
        }
    }
}

#[inline]
fn interrupts() -> &'static mut [[InterruptVector; INT_VECTOR_LEN]] {
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
