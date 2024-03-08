use crate::{cpu_ctx, hartid, send_ipi, Task, TaskId};
use rustsbi::spec::binary::SbiRet;
pub struct SbiHSM;
impl rustsbi::Hsm for SbiHSM {
    #[inline]
    fn hart_start(&self, hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
        if cpu_ctx(hartid).hsm.remote().start(Task {
            entry: start_addr,
            args: [opaque, 0, 0, 0, 0, 0, 0, 0],
            task_id: TaskId::new(crate::hartid() as u16, 0),
        }) {
            core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
            send_ipi(hartid);
            SbiRet::success(0)
        } else {
            SbiRet::already_started()
        }
    }

    #[inline]
    fn hart_stop(&self) -> SbiRet {
        unsafe { cpu_ctx(hartid()).hsm.local() }.stop();
        SbiRet::success(0)
    }

    #[inline]
    fn hart_get_status(&self, hartid: usize) -> SbiRet {
        SbiRet::success(cpu_ctx(hartid).hsm.remote().get_status().unwrap() as usize)
    }

    fn hart_suspend(&self, suspend_type: u32, _resume_addr: usize, _opaque: usize) -> SbiRet {
        use sbi_spec::hsm::{HART_SUSPEND_TYPE_NON_RETENTIVE, HART_SUSPEND_TYPE_RETENTIVE};
        if matches!(
            suspend_type,
            HART_SUSPEND_TYPE_NON_RETENTIVE | HART_SUSPEND_TYPE_RETENTIVE
        ) {
            unsafe { cpu_ctx(hartid()).hsm.local() }.suspend();
            riscv::asm::wfi();
            unsafe { cpu_ctx(hartid()).hsm.local() }.resume();

            SbiRet::success(0)
        } else {
            SbiRet::not_supported()
        }
    }
}
