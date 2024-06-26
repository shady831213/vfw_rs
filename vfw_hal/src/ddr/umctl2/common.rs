use super::super::common::*;
use super::regs::*;
use vfw_primitives::{io_read32, io_write32};
pub trait Umctrl2MPhyV2: DdrDriver {
    fn write_ctrl_reg(&self, reg: usize, value: u32) {
        io_write32!(self.ctrl_base() + reg, value)
    }
    fn read_ctrl_reg(&self, reg: usize) -> u32 {
        io_read32!(self.ctrl_base() + reg)
    }
    fn set_ctrl_reg(&self, reg: usize, value: u32) {
        self.write_ctrl_reg(reg, self.read_ctrl_reg(reg) | value)
    }
    fn clr_ctrl_reg(&self, reg: usize, value: u32) {
        self.write_ctrl_reg(reg, self.read_ctrl_reg(reg) & !value)
    }
    fn write_phy_reg(&self, reg: usize, value: u32) {
        io_write32!(self.phy_base() + reg, value)
    }
    fn read_phy_reg(&self, reg: usize) -> u32 {
        io_read32!(self.phy_base() + reg)
    }
    fn set_phy_reg(&self, reg: usize, value: u32) {
        self.write_phy_reg(reg, self.read_phy_reg(reg) | value)
    }
    fn clr_phy_reg(&self, reg: usize, value: u32) {
        self.write_phy_reg(reg, self.read_phy_reg(reg) & !value)
    }
    fn set_sw_done(&self) {
        self.write_ctrl_reg(UDDRC_SWCTL, 1);
        loop {
            if self.read_ctrl_reg(UDDRC_SWSTAT) == 1 {
                return;
            }
        }
    }
    fn clr_sw_done(&self) {
        self.write_ctrl_reg(UDDRC_SWCTL, 0);
    }

    fn mr_wr(&self, reg: usize, value: u32) {
        loop {
            if self.read_ctrl_reg(UDDRC_MRSTAT) & UDDRC_MRSTAT_MR_WR_BUSY == 0 {
                break;
            }
        }
        self.write_ctrl_reg(
            UDDRC_MRCTRL1,
            (0xff00u32 & ((reg as u32) << 8)) | (0xff & value),
        );
        self.write_ctrl_reg(UDDRC_MRCTRL0, UDDRC_MRCTRL0_MR_RANK(0xf));
        self.write_ctrl_reg(
            UDDRC_MRCTRL0,
            UDDRC_MRCTRL0_MR_RANK(0xf) | UDDRC_MRCTRL0_MR_WR,
        );
        loop {
            if self.read_ctrl_reg(UDDRC_MRCTRL0) & UDDRC_MRCTRL0_MR_WR == 0 {
                break;
            }
        }
    }

    fn div_ratio(&self, n_t: u32) -> u32 {
        if self.freq_ratio2() {
            (n_t + 1) >> 1
        } else {
            n_t
        }
    }

    fn ctrl_base(&self) -> usize;
    fn phy_base(&self) -> usize;
    fn freq_ratio2(&self) -> bool {
        true
    }

    fn skip_train(&self) -> bool {
        false
    }
    fn static_cfg(&self);
    fn sdram_init_cfg(&self);
    fn ecc_cfg(&self);
    fn dfi_cfg(&self);
    fn cfg_timing(&self);
    fn address_map_cfg(&self);
    fn enter_phy_init(&self) -> (u32, u32) {
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 4
        let pwrctl = self.read_ctrl_reg(UDDRC_PWRCTL);
        let rfshctl3 = self.read_ctrl_reg(UDDRC_RFSHCTL3);
        self.clr_ctrl_reg(
            UDDRC_PWRCTL,
            UDDRC_PWRCTL_POWERDOWN_EN
                | UDDRC_PWRCTL_SELFREF_EN
                | UDDRC_PWRCTL_EN_DFI_DRAM_CLK_DISABLE,
        );
        self.clr_ctrl_reg(UDDRC_RFSHCTL3, UDDRC_RFSHCTL3_DIS_AUTO_REFRESH);

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 5-7
        self.clr_sw_done();
        //go to p0 and skip_retrain and clear dfi_init_complete_en
        self.clr_ctrl_reg(UDDRC_DFIMISC, UDDRC_DFIMISC_DFI_INIT_COMPLETE_EN);
        self.set_sw_done();
        (pwrctl, rfshctl3)
    }
    fn phy_init(&self);
    fn sdram_init(&self) {
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 15-17
        self.clr_sw_done();
        self.set_ctrl_reg(UDDRC_DFIMISC, UDDRC_DFIMISC_DFI_INIT_START);
        self.set_sw_done();

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 18
        loop {
            if self.read_ctrl_reg(UDDRC_DFISTAT) & UDDRC_DFISTAT_DFI_INIT_COMPLETE == 1 {
                break;
            }
        }

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 19 - 20
        self.clr_sw_done();
        self.clr_ctrl_reg(UDDRC_DFIMISC, UDDRC_DFIMISC_DFI_INIT_START);
    }
    fn fix_timing(&self) {}
    fn wait_init_done(&self, pwrctl: u32, rfshctl3: u32) {
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 22
        self.set_ctrl_reg(UDDRC_DFIMISC, UDDRC_DFIMISC_DFI_INIT_COMPLETE_EN);

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 23
        self.clr_ctrl_reg(UDDRC_PWRCTL, UDDRC_PWRCTL_SELFREF_SW);

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 24
        self.set_sw_done();

        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 25
        loop {
            if self.read_ctrl_reg(UDDRC_STAT) & UDDRC_STAT_OPERATING_MODE_MSK
                == UDDRC_STAT_OPERATING_MODE_NORMAL
            {
                break;
            }
        }
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 26
        //recovery pwrctl and rfshctl3
        self.write_ctrl_reg(UDDRC_PWRCTL, pwrctl & !UDDRC_PWRCTL_SELFREF_SW);
        self.write_ctrl_reg(UDDRC_RFSHCTL3, rfshctl3);
    }
    fn ports_cfg(&self);
    fn schd_cfg(&self);
    fn reset_core(&self);
    fn dereset_core(&self);
    fn post_init_cfg(&self) {}
    fn init_umctrl2(&self) {
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 1
        self.reset_core();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 2
        //config bl and ranks, static_cfg can override it
        self.write_ctrl_reg(
            UDDRC_MSTR,
            UDDRC_MSTR_BURST_RDWR(((self.layout().bl as usize) >> 1) as u32)
                | self
                    .layout()
                    .ranks
                    .map(|r| UDDRC_MSTR_ACTIVE_RANKS(((1 << (r as usize)) - 1) as u32))
                    .unwrap_or(0),
        );
        //static cfgs
        self.static_cfg();
        self.sdram_init_cfg();
        self.ecc_cfg();
        self.dfi_cfg();
        self.cfg_timing();
        self.address_map_cfg();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2: Note 1
        if self.skip_train() {
            self.clr_ctrl_reg(UDDRC_INIT0, UDDRC_INIT0_SKIP_DRAM_INIT_MSK);
            self.clr_ctrl_reg(UDDRC_PWRCTL, UDDRC_PWRCTL_SELFREF_SW);
        } else {
            self.set_ctrl_reg(UDDRC_INIT0, UDDRC_INIT0_SKIP_DRAM_INIT_MSK);
            self.set_ctrl_reg(UDDRC_PWRCTL, UDDRC_PWRCTL_SELFREF_SW);
        }
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 3
        self.dereset_core();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 4-7
        let (pwrctl, rfshctl3) = self.enter_phy_init();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 8-14
        self.phy_init();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 15-20
        self.sdram_init();
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 21
        if !self.skip_train() {
            self.fix_timing();
        }
        // ----DWC_ddr_umctl2 and Memory Initialization with LPDDR4 mPHY_v2:step 22-26
        self.wait_init_done(pwrctl, rfshctl3);
        self.post_init_cfg();
        // other operations
        self.ports_cfg();
        self.schd_cfg();
    }
    fn inline_ecc(&self) {
        self.write_ctrl_reg(
            UDDRC_ECCCFG0,
            (self.read_ctrl_reg(UDDRC_ECCCFG0) & !UDDRC_ECCCFG0_ECC_MODE_MSK)
                | UDDRC_ECCCFG0_ECC_MODE(UDDRC_ECCCFG0_ECC_MODE_ECCENABLED),
        );
    }
    fn inline_ecc_poison(&self) {
        self.write_ctrl_reg(
            UDDRC_ECCCFG1,
            self.read_ctrl_reg(UDDRC_ECCCFG1) & !UDDRC_ECCCFG1_ECC_REGION_PARITY_LOCK,
        );
    }
    fn scrub_init(&self, range: core::ops::Range<u64>, pat: u64) {
        let htif_range_start = range.start >> (self.layout().data_width() as u64);
        let htif_range_end = (range.end - 1) >> (self.layout().data_width() as u64);
        // Priming the entire DDR space in inline_ecc mode
        self.write_ctrl_reg(
            UDDRC_SBRCTL,
            UDDRC_SBRCTL_SCRUB_MODE
                    | UDDRC_SBRCTL_SCRUB_BURST(1) // 8 Burst for 1 block
                    | UDDRC_SBRCTL_SCRUB_INTERVAL(0), // Back to back write for Priming DDR
        );
        self.write_ctrl_reg(UDDRC_SBRWDATA0, pat as u32);
        self.write_ctrl_reg(UDDRC_SBRWDATA1, (pat >> 32) as u32);
        // Config the scrub address range. Address is HIF address
        // Start from 0
        self.write_ctrl_reg(UDDRC_SBRSTART0, htif_range_start as u32);
        self.write_ctrl_reg(UDDRC_SBRSTART1, (htif_range_start >> 32) as u32);

        self.write_ctrl_reg(UDDRC_SBRRANGE0, htif_range_end as u32);
        self.write_ctrl_reg(UDDRC_SBRRANGE1, (htif_range_end >> 32) as u32);

        //Enable the Scrubber
        self.set_ctrl_reg(UDDRC_SBRCTL, UDDRC_SBRCTL_SCRUB_EN);

        //Polling scrub done
        loop {
            let sbrstat = self.read_ctrl_reg(UDDRC_SBRSTAT);
            if ((sbrstat & UDDRC_SBRSTAT_SCRUB_DONE) != 0)
                && ((sbrstat & UDDRC_SBRSTAT_SCRUB_BUSY) == 0)
            {
                break;
            }
        }
        // Disable scrub
        self.write_ctrl_reg(UDDRC_SBRCTL, 0x0);
    }
    fn load_phy_fw(&self, base: usize, offset: usize, fw: &[u16]);
    fn handle_training_status(&self, status: u8) -> bool {
        match status {
            0x00 => crate::println!("[Ddr trainning] End of initialization"),
            0x01 => crate::println!("[Ddr trainning] End of fine write leveling"),
            0x02 => crate::println!("[Ddr trainning] End of read enable training"),
            0x03 => crate::println!("[Ddr trainning] End of read delay center optimization"),
            0x04 => crate::println!("[Ddr trainning] End of write delay center optimization"),
            0x05 => {
                crate::println!("[Ddr trainning] End of 2D read delay/voltage center optimization")
            }
            0x06 => {
                crate::println!(
                    "[Ddr trainning] End of 2D write delay /voltage center optimization"
                )
            }
            0x07 => {
                crate::println!("[Ddr trainning] Firmware run has completed");
                return true;
            }
            0x08 => crate::println!("[Ddr trainning] Enter streaming message mode"),
            0x09 => crate::println!("[Ddr trainning] End of max read latency training"),
            0x0a => crate::println!("[Ddr trainning] End of read dq deskew training"),
            0x0b => crate::println!("[Ddr trainning] End of LCDL offset calibration"),
            0x0c => crate::println!(
                "[Ddr trainning] End of LRDIMM Specific training (DWL, MREP, MRD and MWD)",
            ),
            0x0d => crate::println!("[Ddr trainning] End of CA training"),
            0xfd => {
                crate::println!("[Ddr trainning] End of MPR read delay center optimization")
            }
            0xfe => crate::println!("[Ddr trainning] End of Write leveling coarse delay"),
            0xff => {
                crate::println!("[Ddr trainning] FATAL ERROR.");
                panic!()
            }
            _ => {
                crate::println!("[Ddr trainning] unknown status {:X}.", status);
                panic!()
            }
        }
        false
    }
}
