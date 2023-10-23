use super::sbi_boot;
use crate::arch::FlowContext;
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
