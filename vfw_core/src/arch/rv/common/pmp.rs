pub const PMP_R: u8 = 0x01;
pub const PMP_W: u8 = 0x02;
pub const PMP_X: u8 = 0x04;

const PMP_CFG_OFF: u16 = 0x3a0;
const PMP_ADDR_OFF: u16 = 0x3b0;

pub const fn pmp_cfg_csr(id: usize) -> u16 {
    PMP_CFG_OFF + (id as u16 >> core::mem::size_of::<usize>().trailing_zeros())
}

pub const fn pmp_addr_csr(id: usize) -> u16 {
    PMP_ADDR_OFF + id as u16
}

pub const fn pmp_cfg_mask(id: usize) -> usize {
    pmp_cfg_enc(id, 0xff)
}

pub const fn pmp_cfg_enc(id: usize, value: u8) -> usize {
    (value as usize) << ((id & (core::mem::size_of::<usize>() - 1)) << 3)
}

pub const fn pmp_addr_na(addr: usize, size: usize) -> usize {
    assert!(size.is_power_of_two());
    assert!(size.trailing_zeros() <= addr.trailing_zeros());
    let mask = (size - 1) >> 3;
    (if size < 8 {
        addr >> 2
    } else {
        (addr >> 3) << 1
    } + mask)
}

#[macro_export]
macro_rules! pmp_cfg_na {
    ($id:expr, $addr: expr, $size: expr, $priv: expr) => {{
        const PMP_NA4: u8 = 0x10;
        const PMP_NAPOT: u8 = 0x18;
        crate::clr_csr!(pmp_cfg_csr($id), pmp_cfg_mask($id));
        crate::write_csr!(pmp_addr_csr($id), pmp_addr_na($addr, $size));
        crate::set_csr!(
            pmp_cfg_csr($id),
            pmp_cfg_enc($id, if $size < 8 { PMP_NA4 } else { PMP_NAPOT } | $priv)
        );
    }};
}

#[macro_export]
macro_rules! pmp_cfg_tor {
    ($id:expr, $top: expr, $priv: expr) => {{
        const PMP_TOR: u8 = 0x08;
        crate::clr_csr!(pmp_cfg_csr($id), pmp_cfg_mask($id));
        crate::write_csr!(pmp_addr_csr($id), $top as usize >> 2);
        crate::set_csr!(pmp_cfg_csr($id), pmp_cfg_enc($id, PMP_TOR | $priv));
    }};
}

#[macro_export]
macro_rules! pmp_disable {
    ($id:expr) => {{
        const PMP_A: u8 = 0x18;
        crate::clr_csr!(pmp_cfg_csr($id), pmp_cfg_enc($id, PMP_A));
    }};
}

#[macro_export]
macro_rules! pmp_lock {
    ($id:expr) => {{
        const PMP_L: u8 = 0x80;
        crate::set_csr!(pmp_cfg_csr($id), pmp_cfg_enc($id, PMP_L));
    }};
}

#[macro_export]
macro_rules! pmp_unlock {
    ($id:expr) => {{
        const PMP_L: u8 = 0x80;
        crate::clr_csr!(pmp_cfg_csr($id), pmp_cfg_enc($id, PMP_L));
        ((crate::read_csr!(pmp_cfg_csr($id)) >> (($id & 0x3) << 3)) as usize & PMP_L) == 0
    }};
}
