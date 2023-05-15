use super::super::super::trap::TrapFrame;
use super::super::access::*;
use super::SbiHandlerError;
use riscv::register::{mepc, mtval};

pub fn misaligned_store_handler(trap_frame: &mut TrapFrame) -> Result<(), SbiHandlerError> {
    let vaddr = mepc::read();
    let ins = unsafe { get_insn(vaddr) };
    let src = ((ins >> 20) & 0x1f) as u8;
    let data = trap_frame.get(src);
    let addr = mtval::read() as usize;
    let func = ((ins >> 12) & 0x7) as usize;
    if ins & 0x7f == 0b0100011 {
        //sw, sh, sd
        unsafe {
            match func {
                1 => {
                    for i in 0..1 {
                        sbi_io_write8(addr + i, (data >> (i << 3)) as u8);
                    }
                }
                2 => match addr.trailing_zeros() {
                    0 => {
                        for i in 0..4 {
                            sbi_io_write8(addr + i, (data >> (i << 3)) as u8);
                        }
                    }
                    1 => {
                        for i in 0..2 {
                            sbi_io_write16(addr + (i << 1), (data >> (i << 4)) as u16);
                        }
                    }
                    _ => {}
                },
                3 => {
                    #[cfg(target_pointer_width = "64")]
                    match addr.trailing_zeros() {
                        0 => {
                            for i in 0..8 {
                                sbi_io_write8(addr + i, (data >> (i << 3)) as u8);
                            }
                        }
                        1 => {
                            for i in 0..4 {
                                sbi_io_write16(addr + (i << 1), (data >> (i << 4)) as u16);
                            }
                        }
                        2 => {
                            for i in 0..2 {
                                sbi_io_write32(addr + (i << 2), (data >> (i << 5)) as u32);
                            }
                        }
                        _ => {}
                    }
                    #[cfg(target_pointer_width = "32")]
                    {
                        return Err(SbiHandlerError::Unhandled);
                    }
                }
                _ => return Err(SbiHandlerError::Unhandled),
            }
        }
        mepc::write(mepc::read().wrapping_add(4));
        Ok(())
    } else {
        Err(SbiHandlerError::Unhandled)
    }
}

pub fn misaligned_load_handler(trap_frame: &mut TrapFrame) -> Result<(), SbiHandlerError> {
    let vaddr = mepc::read();
    let ins = unsafe { get_insn(vaddr) };
    let addr = mtval::read() as usize;
    if ins & 0x7f == 0b0000011 {
        //lw, lwu, lh, lhu, ld
        let rd = ((ins >> 7) & 0x1f) as u8;
        let func = ((ins >> 12) & 0x3) as usize;
        let signed = ((ins >> 14) & 0x1) == 1;
        let data = unsafe {
            match func {
                1 => {
                    let mut data: usize = 0;
                    for i in 0..2 {
                        data |= (sbi_io_read8(addr + i) as usize & 0xff) << (i << 3);
                    }
                    if signed && (data >> 15) == 1 {
                        data |= ((-1isize as usize) >> 16) << 16
                    }
                    data
                }
                2 => {
                    let mut data: usize = 0;
                    match addr.trailing_zeros() {
                        0 => {
                            for i in 0..4 {
                                data |= (sbi_io_read8(addr + i) as usize & 0xff) << (i << 3);
                            }
                        }
                        1 => {
                            for i in 0..2 {
                                data |=
                                    (sbi_io_read16(addr + (i << 1)) as usize & 0xffff) << (i << 4);
                            }
                        }
                        _ => {}
                    }
                    #[cfg(target_pointer_width = "64")]
                    if signed && (data >> 31) == 1 {
                        data |= ((-1isize as usize) >> 31) << 31
                    }
                    data
                }
                3 => {
                    #[cfg(target_pointer_width = "64")]
                    {
                        let mut data: usize = 0;
                        match addr.trailing_zeros() {
                            0 => {
                                for i in 0..8 {
                                    data |= (sbi_io_read8(addr + i) as usize & 0xff) << (i << 3);
                                }
                            }
                            1 => {
                                for i in 0..4 {
                                    data |= (sbi_io_read16(addr + (i << 1)) as usize & 0xffff)
                                        << (i << 4);
                                }
                            }
                            2 => {
                                for i in 0..2 {
                                    data |= (sbi_io_read32(addr + (i << 2)) as usize & 0xffffffff)
                                        << (i << 5);
                                }
                            }
                            _ => {}
                        }
                        data
                    }
                    #[cfg(target_pointer_width = "32")]
                    {
                        return Err(SbiHandlerError::Unhandled);
                    }
                }
                _ => 0,
            }
        };
        trap_frame.update(rd, data);
        mepc::write(mepc::read().wrapping_add(4));
        Ok(())
    } else if ins & 0xe003 == 0x4000 {
        // c.lw
        let rd = ((ins >> 2) & 0x7) as u8;
        let mut data: usize = 0;
        unsafe {
            match addr.trailing_zeros() {
                0 => {
                    for i in 0..4 {
                        data |= (sbi_io_read8(addr + i) as usize & 0xff) << (i << 3);
                    }
                }
                1 => {
                    for i in 0..2 {
                        data |= (sbi_io_read16(addr + (i << 1)) as usize & 0xffff) << (i << 4);
                    }
                }
                _ => {}
            }
        };
        trap_frame.update(rd, data);
        mepc::write(mepc::read().wrapping_add(2));
        Ok(())
    } else if ins & 0xe003 == 0x6000 {
        #[cfg(target_pointer_width = "64")]
        {
            // c.ld
            let rd = ((ins >> 2) & 0x7) as u8;
            let mut data: usize = 0;
            unsafe {
                match addr.trailing_zeros() {
                    0 => {
                        for i in 0..8 {
                            data |= (sbi_io_read8(addr + i) as usize & 0xff) << (i << 3);
                        }
                    }
                    1 => {
                        for i in 0..4 {
                            data |= (sbi_io_read16(addr + (i << 1)) as usize & 0xffff) << (i << 4);
                        }
                    }
                    2 => {
                        for i in 0..2 {
                            data |=
                                (sbi_io_read32(addr + (i << 2)) as usize & 0xffffffff) << (i << 5);
                        }
                    }
                    _ => {}
                }
            }
            trap_frame.update(rd, data);
            mepc::write(mepc::read().wrapping_add(2));
            Ok(())
        }
        #[cfg(target_pointer_width = "32")]
        {
            //c.flw
            Err(SbiHandlerError::Unhandled)
        }
    } else {
        Err(SbiHandlerError::Unhandled)
    }
}
