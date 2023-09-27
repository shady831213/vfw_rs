#[cfg(not(feature = "stack_non_priv"))]
core::arch::global_asm!(include_str!("crt_priv_stack.S"));
#[cfg(feature = "stack_non_priv")]
core::arch::global_asm!(include_str!("crt_non_priv_stack.S"));

#[naked]
#[no_mangle]
pub(crate) unsafe extern "C" fn reuse_stack_for_trap() {
    core::arch::asm!(
        "   call t1, {move_stack}
            ret
        ",
        move_stack          =   sym fast_trap::reuse_stack_for_trap,
        options(noreturn),
    )
}
