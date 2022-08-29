use crate::hal::ddr::common::DdrSpeed;
#[allow(non_snake_case)]
pub struct Lpddr4Timing {
    pub tRASmin: f32,   //by ns
    pub tRAScmin: u32,  //by cycle of CK
    pub tRASmax: f32,   //by ns
    pub tFAW: f32,      //by ns
    pub tRC: f32,       //by ns
    pub tRTP: f32,      //by ns
    pub tXP: f32,       //by ns
    pub tXPc: u32,      //by cycle of CK
    pub tWR: f32,       //by ns
    pub tWRc: u32,      //by cycle of CK
    pub tWTR: f32,      //by ns
    pub tWTRc: u32,     //by cycle of CK
    pub tDQSCKmax: f32, //by ns
    pub tMRD: f32,      //by ns
    pub tMRDc: u32,     //by cycle of CK
    pub tMRW: f32,      //by ns
    pub tMRWc: u32,     //by cycle of CK
    pub tRP: f32,       //by ns
    pub tRPc: u32,      //by cycle of CK
    pub tRRD: f32,      //by ns
    pub tCCD: u32,      //by cycle of CK
    pub tRCD: f32,      //by ns
    pub tRCDc: u32,     //by cycle of CK
    pub tPPD: u32,      //by cycle of CK
    pub tCCDMW: u32,    //by cycle of CK
    pub tODToff: u32,   //by cycle of CK
    pub tRFCab: f32,    //by ns
    pub tREFIab: f32,   //by ns
}
impl Lpddr4Timing {
    const fn ceil(&self, data: f32) -> u32 {
        let floor = self.floor(data);
        if (floor as f32) < data {
            floor + 1
        } else {
            floor
        }
    }
    const fn floor(&self, data: f32) -> u32 {
        data as u32
    }
    const fn ceil_with_min(&self, data: f32, min: u32) -> u32 {
        let data = self.ceil(data);
        if data > min {
            data
        } else {
            min
        }
    }
    pub const fn n_ras_min(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tRASmin / tck, self.tRAScmin)
    }
    pub const fn n_ras_max(&self, tck: f32) -> u32 {
        self.floor(self.tRASmax / tck)
    }
    pub const fn n_faw(&self, tck: f32) -> u32 {
        self.ceil(self.tFAW / tck)
    }
    pub const fn n_rc(&self, tck: f32) -> u32 {
        self.ceil(self.tRC / tck)
    }
    pub const fn n_rtp(&self, tck: f32) -> u32 {
        self.ceil(self.tRTP / tck)
    }
    pub const fn n_xp(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tXP / tck, self.tXPc)
    }
    pub const fn n_wtr(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tWTR / tck, self.tWTRc)
    }
    pub const fn n_wr(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tWR / tck, self.tWRc)
    }
    pub const fn n_dqsck_max(&self, tck: f32) -> u32 {
        self.ceil(self.tDQSCKmax / tck)
    }
    pub const fn n_mrd(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tMRD / tck, self.tMRDc)
    }
    pub const fn n_mrw(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tMRW / tck, self.tMRWc)
    }
    pub const fn n_rp(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tRP / tck, self.tRPc)
    }
    pub const fn n_rrd(&self, tck: f32) -> u32 {
        self.ceil(self.tRRD / tck)
    }
    pub const fn n_rcd(&self, tck: f32) -> u32 {
        self.ceil_with_min(self.tRCD / tck, self.tRCDc)
    }
    pub const fn n_rfc_ab(&self, tck: f32) -> u32 {
        self.ceil(self.tRFCab / tck)
    }
    pub const fn n_refi_ab(&self, tck: f32) -> u32 {
        self.ceil(self.tREFIab / tck)
    }
}

pub struct Lpddr4Cfg {
    pub wr_preamble: bool,
    pub rd_preamble: bool,
    pub rd_postamble: bool,
    pub wr_postamble: bool,
    pub wl_set_b: bool,
    pub dis_write_level: bool,
    pub pullup_vddq3: bool,
    pub pprp_disable: bool,
    pub pulldown_ds: u16,
    pub dbi_rd: bool,
    pub dbi_wr: bool,
}

impl Lpddr4Cfg {
    //mr1[1:0] reserved
    //mr1[2] wr preamble 2tck
    //mr1[3] fixed rd preamble
    //mr1[6:4] wr2precharge
    //mr1[7] rd postamble 0.5tck
    pub const fn mr1(&self, speed: &DdrSpeed) -> u16 {
        ((self.wr_preamble as u16) << 2)
            | ((self.rd_preamble as u16) << 3)
            | (self.speed_code(speed) << 4)
            | ((self.rd_postamble as u16) << 7)
    }
    //mr2[2:0]  Read Latancy
    //mr2[5:3]  Write Latancy
    //mr2[6]  WL Set"A"/"B"
    //mr2[7] Disable Write Leveling
    pub const fn mr2(&self, speed: &DdrSpeed) -> u16 {
        self.speed_code(speed)
            | (self.speed_code(speed) << 3)
            | ((self.wl_set_b as u16) << 6)
            | ((self.dis_write_level as u16) << 6)
    }

    //mr3[0] pull up Vddq
    //mr3[1] Write Postamble
    //mr3[2] post package repair protection disable
    //mr3[5:3] pull down drive strength
    //mr3[6] DBI-RD enable
    //mr3[7] DBI_WR enable
    pub const fn mr3(&self) -> u16 {
        (self.pullup_vddq3 as u16)
            | ((self.wr_postamble as u16) << 1)
            | ((self.pprp_disable as u16) << 2)
            | ((self.pulldown_ds as u16 & 0x7) << 3)
            | ((self.dbi_rd as u16) << 6)
            | ((self.dbi_wr as u16) << 7)
    }

    const fn speed_code(&self, speed: &DdrSpeed) -> u16 {
        match speed {
            DdrSpeed::Spd1600 => 0x6,
            DdrSpeed::Spd800 => 0x2,
            DdrSpeed::Spd533 | DdrSpeed::Spd500 | DdrSpeed::Spd400 => 0x1,
            DdrSpeed::Spd266 => 0x0,
        }
    }

    pub const fn bl(&self) -> usize {
        16
    }

    pub const fn half_bl(&self) -> usize {
        self.bl() >> 1
    }

    pub const fn rl(&self, speed: &DdrSpeed) -> usize {
        match self.speed_code(speed) {
            0 => 6,
            1 => 12,
            2 => 16,
            3 => 22,
            4 => 28,
            5 => 32,
            6 => 36,
            7 => 40,
            _ => 6,
        }
    }

    pub const fn wl(&self, speed: &DdrSpeed) -> usize {
        let code = ((self.wl_set_b as u16) << 3) | self.speed_code(speed);
        match code {
            // Set-A
            0 => 4,
            1 => 6,
            2 => 8,
            3 => 10,
            4 => 12,
            5 => 14,
            6 => 16,
            7 => 18,
            // Set-B
            8 => 4,
            9 => 8,
            10 => 12,
            11 => 18,
            12 => 22,
            13 => 26,
            14 => 30,
            15 => 34,
            _ => 4,
        }
    }
}
