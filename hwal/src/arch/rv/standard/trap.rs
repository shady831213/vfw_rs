use crate::arch::rv::riscv;
use crate::arch::rv::trap::*;
#[cfg(target_pointer_width = "32")]
global_asm!(include_str!("../rv32.S"));

global_asm!(include_str!("trap.S"));

#[no_mangle]
extern "C" fn start_trap_rust(trap_frame: &mut TrapFrame) {
    use riscv::register::mcause;
    unsafe {
        let cause = mcause::read();
        if cause.is_exception() {
            __Exception.handle(trap_frame);
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

use riscv::register::mtvec::{self, TrapMode};
pub fn init_trap(mode: TrapMode) {
    extern "C" {
        static _vector_table: usize;
    }
    let m_vector_table = unsafe { &_vector_table } as *const usize as usize;
    unsafe {
        mtvec::write(m_vector_table, mode);
    }
}

pub unsafe fn register_exception_handler(f: unsafe extern "C" fn(trap_frame: &mut TrapFrame)) {
    __Exception.handler = f;
}

pub unsafe fn register_int_handler(cause: Interrupt, f: unsafe extern "C" fn()) {
    let code = cause as usize;
    if code < __INTERRUPTS.len() {
        __INTERRUPTS[code].handler = f;
    }
}

#[no_mangle]
static mut __INTERRUPTS: [InterruptVector; 12] = [const { InterruptVector { reserved: 0 } }; 12];

#[no_mangle]
static mut __Exception: ExceptionVector = ExceptionVector { reserved: 0 };
