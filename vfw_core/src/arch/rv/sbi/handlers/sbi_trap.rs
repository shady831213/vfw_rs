use super::sbi_boot;
use crate::{
    arch::{trap_handler, FlowContext},
    clear_ipi, cpu_ctx, hartid, wait_ipi,
};
use riscv::register::{
    mcause::{self, Exception as E, Trap},
    mstatus,
};
pub fn sbi_call<
    T: rustsbi::Timer,
    I: rustsbi::Ipi,
    R: rustsbi::Fence,
    H: rustsbi::Hsm,
    S: rustsbi::Reset,
    P: rustsbi::Pmu,
    F: Fn(u8) -> usize,
>(
    sbi: &mut rustsbi::RustSBI<T, I, R, H, S, P>,
    printer: F,
    regs: &mut FlowContext,
) -> bool {
    use sbi_spec::{base, hsm, legacy};
    let mut ret = sbi.handle_ecall(
        regs.a[7],
        regs.a[6],
        [
            regs.a[0], regs.a[1], regs.a[2], regs.a[3], regs.a[4], regs.a[5],
        ],
    );
    if ret.is_ok() {
        match (regs.a[7], regs.a[6]) {
            (hsm::EID_HSM, hsm::HART_STOP) => {
                return false;
            }
            (hsm::EID_HSM, hsm::HART_SUSPEND)
                if matches!(regs.a[0] as u32, hsm::HART_SUSPEND_TYPE_NON_RETENTIVE) =>
            {
                sbi_boot(regs, regs.a[1], regs.a[2]);
                return true;
            }
            (base::EID_BASE, base::PROBE_EXTENSION)
                if matches!(regs.a[0], legacy::LEGACY_CONSOLE_PUTCHAR) =>
            {
                ret.value = 1;
            }
            _ => {}
        }
    } else {
        match regs.a[7] {
            legacy::LEGACY_CONSOLE_PUTCHAR => {
                ret.error = printer(regs.a[0] as u8);
                ret.value = regs.a[1];
            }
            _ => {}
        }
    }
    regs.a[0] = ret.error;
    regs.a[1] = ret.value;
    regs.pc += 4;
    true
}

pub fn sbi_trap_loop<
    T: rustsbi::Timer,
    I: rustsbi::Ipi,
    R: rustsbi::Fence,
    H: rustsbi::Hsm,
    S: rustsbi::Reset,
    P: rustsbi::Pmu,
    F: Fn(u8) -> usize,
>(
    sbi: &mut rustsbi::RustSBI<T, I, R, H, S, P>,
    printer: F,
    regs: &mut FlowContext,
) {
    loop {
        match unsafe { cpu_ctx(hartid()).hsm.local() }.start() {
            Ok(task) => {
                unsafe {
                    mstatus::set_mpp(mstatus::MPP::Supervisor);
                    mstatus::set_mpie();
                }
                break sbi_boot(regs, task.entry, task.args[0]);
            }
            Err(s) if s as usize == sbi_spec::hsm::HART_STOP => {
                wait_ipi();
                clear_ipi(hartid());
            }
            _ => match mcause::read().cause() {
                Trap::Exception(E::SupervisorEnvCall) => {
                    if sbi_call(sbi, &printer, regs) {
                        break;
                    }
                }
                _ => {
                    break trap_handler(regs);
                }
            },
        }
    }
}

#[macro_export]
macro_rules! sbi_trap_handler(
    (@ ($($dec:ident)*) ($($lit:literal)*) $(#[$attribute:meta])* $vis:vis $name:ident $($tt:tt)*) => {
        paste::paste!{
        #[naked]
        $vis extern "C" fn $name(_regs: &mut crate::FlowContext) {
            unsafe {
                core::arch::asm!("mv sp, tp",
                "j {handler}",
                handler = sym [<_ $name>],
                options(noreturn)
                )
            }
        }

        $(#[$attribute])* $vis $($dec)* $($lit)* fn [<_ $name>] $($tt)*
        }
    };
    ($(#[$attribute:meta])* $vis:vis fn $name:ident $($tt:tt)*) => {
        crate::sbi_trap_handler!(@ () () $(#[$attribute])* $vis $name $($tt)*);
    };
    ($(#[$attribute:meta])* $vis:vis unsafe fn $name:ident $($tt:tt)*) => {
        crate::sbi_trap_handler!(@ (unsafe) () $(#[$attribute])* $vis $name $($tt)*);
    };
    ($(#[$attribute:meta])* $vis:vis extern "C" fn $name:ident $($tt:tt)*) => {
        crate::sbi_trap_handler!(@ (extern) ("C") $(#[$attribute])* $vis $name $($tt)*);
    };
    ($(#[$attribute:meta])* $vis:vis unsafe extern "C" fn $name:ident $($tt:tt)*) => {
        crate::sbi_trap_handler!(@ (unsafe extern) ("C") $(#[$attribute])* $vis $name $($tt)*);
    };
);
