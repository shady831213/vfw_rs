// use super::SbiHandlerError;
// use fast_trap::FastContext;
// use riscv::register::mepc;
// pub fn sbi_call_handler(
//     mut ctx: FastContext,
//     a1: usize,
//     a2: usize,
//     a3: usize,
//     a4: usize,
//     a5: usize,
//     a6: usize,
//     a7: usize,
// ) -> Result<(), SbiHandlerError> {
//     let params = [ctx.a0(), a1, a2, a3, a4, a5];
//     // Call RustSBI procedure
//     let ans = rustsbi::ecall(a7, a6, params);
//     // handle get_char
//     if a7 as u32 == 0x2 && ans.value == 0xff {
//         ctx.regs().a[0] = ans.error;
//         ctx.regs().a[1] = -1isize as usize;
//     } else {
//         // Return the return value to TrapFrame
//         ctx.regs().a[0] = ans.error;
//         ctx.regs().a[1] = ans.value;
//     }
//     // Skip ecall instruction
//     mepc::write(mepc::read().wrapping_add(4));
//     Ok(())
// }
