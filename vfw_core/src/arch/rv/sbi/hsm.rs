use super::{SbiStacks, Supervisor};
use crate::send_ipi;
use core::ptr::NonNull;
use rustsbi::spec::binary::SbiRet;
pub struct SbiStackHSM<const SIZE: usize, const NUM: usize>(NonNull<SbiStacks<SIZE, NUM>>);
unsafe impl<const SIZE: usize, const NUM: usize> Send for SbiStackHSM<SIZE, NUM> {}
unsafe impl<const SIZE: usize, const NUM: usize> Sync for SbiStackHSM<SIZE, NUM> {}
impl<const SIZE: usize, const NUM: usize> SbiStackHSM<SIZE, NUM> {
    pub const fn new(stack: NonNull<SbiStacks<SIZE, NUM>>) -> Self {
        Self(stack)
    }
}
impl<const SIZE: usize, const NUM: usize> rustsbi::Hsm for SbiStackHSM<SIZE, NUM> {
    #[inline]
    fn hart_start(&self, hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
        match unsafe { &mut *self.0.as_ptr() }.remote_hsm(hartid) {
            Some(remote) => {
                if remote.start(Supervisor { start_addr, opaque }) {
                    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                    send_ipi(hartid);
                    SbiRet::success(0)
                } else {
                    SbiRet::already_started()
                }
            }
            None => SbiRet::invalid_param(),
        }
    }

    #[inline]
    fn hart_stop(&self) -> SbiRet {
        unsafe { &mut *self.0.as_ptr() }.local_hsm().stop();
        SbiRet::success(0)
    }

    #[inline]
    fn hart_get_status(&self, hartid: usize) -> SbiRet {
        match unsafe { &mut *self.0.as_ptr() }.remote_hsm(hartid) {
            Some(remote) => SbiRet::success(remote.get_status().unwrap() as usize),
            None => SbiRet::invalid_param(),
        }
    }

    fn hart_suspend(&self, suspend_type: u32, _resume_addr: usize, _opaque: usize) -> SbiRet {
        use rustsbi::spec::hsm::{HART_SUSPEND_TYPE_NON_RETENTIVE, HART_SUSPEND_TYPE_RETENTIVE};
        if matches!(
            suspend_type,
            HART_SUSPEND_TYPE_NON_RETENTIVE | HART_SUSPEND_TYPE_RETENTIVE
        ) {
            unsafe { &mut *self.0.as_ptr() }.local_hsm().suspend();
            unsafe {
                riscv::asm::wfi();
            }
            unsafe { &mut *self.0.as_ptr() }.local_hsm().resume();

            SbiRet::success(0)
        } else {
            SbiRet::not_supported()
        }
    }
}
