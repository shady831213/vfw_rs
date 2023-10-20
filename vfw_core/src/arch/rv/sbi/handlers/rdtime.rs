use super::SbiHandlerError;
use super::*;
use crate::FlowContext;
use riscv::register::{mepc, mstatus, scounteren};
pub trait SbiTimer: rustsbi::Timer {
    fn get_time(&self) -> u64;
}

pub fn rdtime_handler<T: SbiTimer>(
    ctx: &mut FlowContext,
    ins: u32,
    timer: &T,
) -> Result<(), SbiHandlerError> {
    if ins & 0xFFFFF07F == 0xC0102073 {
        // rdtime
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        let counteren = if mstatus::read().mpp() == mstatus::MPP::User {
            scounteren::read().tm()
        } else {
            true
        };
        if counteren {
            let time_usize = timer.get_time() as usize;
            update_reg(ctx, rd, time_usize);
        }
        mepc::write(mepc::read().wrapping_add(4));
        Ok(())
    } else if ins & 0xFFFFF07F == 0xC8102073 {
        // rdtimeh
        let rd = ((ins >> 7) & 0b1_1111) as u8;
        let counteren = if mstatus::read().mpp() == mstatus::MPP::User {
            scounteren::read().tm()
        } else {
            true
        };
        if counteren {
            let time_usize = (timer.get_time() >> 32) as usize;
            update_reg(ctx, rd, time_usize);
        }
        mepc::write(mepc::read().wrapping_add(4));
        Ok(())
    } else {
        Err(SbiHandlerError::Unhandled)
    }
}
