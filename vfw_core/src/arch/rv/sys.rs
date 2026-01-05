pub fn rv_wait_ipi() {
    use riscv::asm::wfi;
    use riscv::register::mie;
    unsafe {
        let flag = crate::save_flag();
        let sie = mie::read().msoft();
        mie::set_msoft();
        wfi();
        if !sie {
            mie::clear_msoft();
        }
        crate::restore_flag(flag);
    }
}

#[macro_export]
macro_rules! read_csr {
    ($csr_number:expr) => {
        {
            unsafe {
                let r: usize;
                core::arch::asm!("csrrs {0}, {1}, x0", out(reg) r, const $csr_number, options(pure, nomem, nostack));
                r
            }
        }
    }
}

#[macro_export]
macro_rules! write_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrw x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! set_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrs x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! clr_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrc x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! get_sp {
    () => {
        {
            unsafe {
                let sp: usize;
                core::arch::asm!("mv {0}, sp", out(reg) sp, options(pure, nomem, nostack));
                sp
            }
        }
    }
}

#[cfg(target_pointer_width = "32")]
mod usize_access {
    #[macro_export]
    macro_rules! load_usize {
        ($reg:expr, $tag:expr) => {
            concat!("lw ", stringify!($reg), ", ", stringify!($tag),)
        };
    }
    #[macro_export]
    macro_rules! fill_usize {
        ($tag:expr, $symbol:expr) => {
            concat!(stringify!($tag), ": ", ".word ", stringify!($symbol))
        };
    }
}

#[cfg(target_pointer_width = "64")]
mod usize_access {
    #[macro_export]
    macro_rules! load_usize {
        ($reg:expr, $tag:expr) => {
            concat!("ld ", stringify!($reg), ", ", stringify!($tag),)
        };
    }
    #[macro_export]
    macro_rules! fill_usize {
        ($tag:expr, $symbol:expr) => {
            concat!(stringify!($tag), ": ", ".dword ", stringify!($symbol))
        };
    }
}

#[macro_export]
macro_rules! get_tp {
    () => {
        {
            unsafe {
                let tp: usize;
                core::arch::asm!("mv {0}, tp", out(reg) tp, options(pure, nomem, nostack));
                tp
            }
        }
    }
}

#[macro_export]
macro_rules! check_stack {
    () => {{
        extern "C" {
            fn stack_start() -> usize;
            fn stack_size() -> usize;
        }
        unsafe {
            let stack_end = stack_start() - stack_size();
            if get_sp!() > stack_start() || get_sp!() <= stack_end {
                panic!(
                    "stack {:#x} is out of range ({:#x} - {:#x})",
                    get_sp!(),
                    stack_start(),
                    stack_end
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! relocation {
    (mut $sym:ident:$t:ty) => {
        unsafe {
            &mut *(crate::relocation!(@do_asm $sym) as *mut $t)
        }
    };
    ($sym:ident:$t:ty) => {
        unsafe {
            &const *(crate::relocation!(@do_asm $sym) as *const $t)
        }
    };
    (@do_asm $sym:ident) => {
        {
                let o: usize;
                core::arch::asm!(
                "
            .option push
            .option norelax
            la {o},{i}
            .option pop", i = sym $sym,
                o = out(reg) o,
                );
                o
        }
    };
}

#[no_mangle]
pub fn wait_mcycle(cnt: usize) {
    let cur = riscv::register::mcycle::read();
    while riscv::register::mcycle::read().wrapping_sub(cur) < cnt {}
}

#[no_mangle]
pub fn mcycle() -> usize {
    riscv::register::mcycle::read()
}

#[no_mangle]
pub fn wait_mcycle64(cnt: u64) {
    let cur = riscv::register::mcycle::read64();
    while riscv::register::mcycle::read64().wrapping_sub(cur) < cnt {}
}

#[no_mangle]
pub fn mcycle64() -> u64 {
    riscv::register::mcycle::read64()
}
