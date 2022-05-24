use super::SbiHandlerError;
use crate::arch::rv::trap::TrapFrame;
use riscv::register::mepc;
pub fn sbi_call_handler(trap_frame: &mut TrapFrame) -> Result<(), SbiHandlerError> {
    let params = [
        trap_frame.a0,
        trap_frame.a1,
        trap_frame.a2,
        trap_frame.a3,
        trap_frame.a4,
        trap_frame.a5,
    ];
    // Call RustSBI procedure
    let ans = rustsbi::ecall(trap_frame.a7, trap_frame.a6, params);
    // Return the return value to TrapFrame
    trap_frame.a0 = ans.error;
    trap_frame.a1 = ans.value;
    // Skip ecall instruction
    mepc::write(mepc::read().wrapping_add(4));
    Ok(())
}
