use super::super::common::DdrSpeed;
use super::super::sdram::lpddr4::*;
use super::common::Umctrl2MPhyV2;
use super::regs;
pub struct Umctl2Lpddr4TimingCfg {
    pub wl: u32,
    pub rl: u32,
    pub half_bl: u32,
    pub n_wr: u32,
    pub n_ras_min: u32,
    pub n_ras_max: u32,
    pub n_faw: u32,
    pub n_rtp: u32,
    pub n_rc: u32,
    pub n_xp: u32,
    pub n_wtr: u32,
    pub n_dqsck_max: u32,
    pub n_mrd: u32,
    pub n_mrw: u32,
    pub n_rp: u32,
    pub n_rrd: u32,
    pub n_ccd: u32,
    pub n_rcd: u32,
    pub n_ppd: u32,
    pub n_ccdmw: u32,
    pub n_odtoff: u32,
    pub n_rfc_ab: u32,
    pub n_refi_ab: u32,
    pub n_zq_long_nop: u32,
    pub n_zq_short_nop: u32,
    pub n_zq_reset_nop: u32,
}

impl Umctl2Lpddr4TimingCfg {
    pub const fn new(
        lpddr4_timing: &Lpddr4Timing,
        lpddr4_cfg: &Lpddr4Cfg,
        speed: &DdrSpeed,
    ) -> Umctl2Lpddr4TimingCfg {
        let tck = speed.t_ck();
        Umctl2Lpddr4TimingCfg {
            wl: lpddr4_cfg.wl(speed) as u32,
            rl: lpddr4_cfg.rl(speed) as u32,
            half_bl: lpddr4_cfg.half_bl() as u32,
            n_wr: lpddr4_timing.n_wr(tck),
            n_ras_min: lpddr4_timing.n_ras_min(tck),
            n_ras_max: lpddr4_timing.n_ras_max(tck),
            n_faw: lpddr4_timing.n_faw(tck),
            n_rtp: lpddr4_timing.n_rtp(tck),
            n_rc: lpddr4_timing.n_rc(tck),
            n_xp: lpddr4_timing.n_xp(tck),
            n_wtr: lpddr4_timing.n_wtr(tck),
            n_dqsck_max: lpddr4_timing.n_dqsck_max(tck),
            n_mrd: lpddr4_timing.n_mrd(tck),
            n_mrw: lpddr4_timing.n_mrw(tck),
            n_rp: lpddr4_timing.n_rp(tck),
            n_rrd: lpddr4_timing.n_rrd(tck),
            n_ccd: lpddr4_timing.tCCD,
            n_rcd: lpddr4_timing.n_rcd(tck),
            n_ppd: lpddr4_timing.tPPD,
            n_ccdmw: lpddr4_timing.tCCDMW,
            n_odtoff: lpddr4_timing.tODToff,
            n_rfc_ab: lpddr4_timing.n_rfc_ab(tck),
            n_refi_ab: lpddr4_timing.n_refi_ab(tck),
            n_zq_long_nop: lpddr4_timing.n_zq_long_nop(tck),
            n_zq_short_nop: lpddr4_timing.n_zq_short_nop(tck),
            n_zq_reset_nop: lpddr4_timing.n_zq_reset_nop(tck),
        }
    }
}

pub trait Umctl2Lpddr4: Umctrl2MPhyV2 {
    fn lpddr4_static_cfg(&self) {
        self.set_ctrl_reg(regs::UDDRC_MSTR, regs::UDDRC_MSTR_LPDDR4);
    }
    fn lpddr4_timing(&self) -> &Umctl2Lpddr4TimingCfg;
    //all neccessary timing cfg refer to 2.20.8 Registers Related to Dynamic SDRAM Constraints
    fn lpddr4_cfg_timing(&self) {
        let timing = self.lpddr4_timing();
        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG0,
            regs::UDDRC_DRAMTMG0_T_RAS_MIN(self.div_ratio(timing.n_ras_min))
                | regs::UDDRC_DRAMTMG0_T_RAS_MAX(self.div_ratio(timing.n_ras_max >> 10))//need div 1024
                | regs::UDDRC_DRAMTMG0_T_FAW(self.div_ratio(timing.n_faw))
                | regs::UDDRC_DRAMTMG0_WR2PRE(self.div_ratio(timing.wl + timing.half_bl + timing.n_wr)),
        ); //(          DRAMTMG0)

        let n_rtp = timing.n_rtp;
        let n_rtp = if n_rtp > 8 { n_rtp - 8 } else { 0 };
        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG1,
            regs::UDDRC_DRAMTMG1_T_RC(self.div_ratio(timing.n_rc))
                | regs::UDDRC_DRAMTMG1_RD2PRE(self.div_ratio(timing.half_bl + n_rtp))
                | regs::UDDRC_DRAMTMG1_T_XP(self.div_ratio(timing.n_xp)),
        ); //(          DRAMTMG1)
        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG2,
            regs::UDDRC_DRAMTMG2_WR2RD(
                self.div_ratio(timing.wl + timing.half_bl + timing.n_wtr + 2),
            ) | regs::UDDRC_DRAMTMG2_RD2WR(
                self.div_ratio(timing.rl + timing.half_bl + timing.n_dqsck_max - timing.wl + 2),
            ) | regs::UDDRC_DRAMTMG2_READ_LATENCY(self.div_ratio(timing.rl))
                | regs::UDDRC_DRAMTMG2_WRITE_LATENCY(self.div_ratio(timing.wl)),
        ); //(          DRAMTMG2)
           // ---- sdram init seq refer to 2.21.6 LPDDR4 Initialization Sequence 6-9
        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG3,
            regs::UDDRC_DRAMTMG3_T_MRD(self.div_ratio(timing.n_mrd))
                | regs::UDDRC_DRAMTMG3_T_MRW(self.div_ratio(timing.n_mrw)),
        ); //(          DRAMTMG3)

        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG4,
            regs::UDDRC_DRAMTMG4_T_RP(self.div_ratio(timing.n_rp))
                | regs::UDDRC_DRAMTMG4_T_RRD(self.div_ratio(timing.n_rrd))
                | regs::UDDRC_DRAMTMG4_T_CCD(self.div_ratio(timing.n_ccd))
                | regs::UDDRC_DRAMTMG4_T_RCD(self.div_ratio(timing.n_rcd)),
        ); //(          DRAMTMG4)
        self.write_ctrl_reg(
            regs::UDDRC_DRAMTMG13,
            regs::UDDRC_DRAMTMG13_T_PPD(self.div_ratio(timing.n_ppd))
                | regs::UDDRC_DRAMTMG13_T_CCD_MW(self.div_ratio(timing.n_ccdmw))
                | regs::UDDRC_DRAMTMG13_ODTLOFF(self.div_ratio(timing.n_odtoff)),
        ); //(         DRAMTMG13)

        self.write_ctrl_reg(
            regs::UDDRC_RFSHTMG,
            regs::UDDRC_RFSHTMG_T_RFC_MIN(self.div_ratio(timing.n_rfc_ab))
                | regs::UDDRC_RFSHTMG_T_RFC_NOM_X32(self.div_ratio(timing.n_refi_ab) >> 5),
        ); //(           RFSHTMG)

        self.write_ctrl_reg(
            regs::UDDRC_DFITMG0,
            regs::UDDRC_DFITMG0_DFI_TPHY_WRLAT(timing.wl-4) // wl - 3 - 1pre
                | regs::UDDRC_DFITMG0_DFI_TPHY_WRDATA(0x2)
                | regs::UDDRC_DFITMG0_DFI_WRDATA_USE_DFI_PHY_CLK
                | regs::UDDRC_DFITMG0_DFI_T_RDDATA_EN(timing.rl -5) // rl - 3 - 2 pre
                | regs::UDDRC_DFITMG0_DFI_RDDATA_USE_DFI_PHY_CLK
                | regs::UDDRC_DFITMG0_DFI_T_CTRL_DELAY(0x3),
        ); //(           DFITMG0)
    }
    fn lpddr4_cfg_zq(&self) {
        // ---- sdram init seq refer to 2.21.6 LPDDR4 Initialization Sequence 11-12
        let timing = self.lpddr4_timing();
        self.write_ctrl_reg(
            regs::UDDRC_ZQCTL0,
            regs::UDDRC_ZQCTL0_T_ZQ_SHORT_NOP(self.div_ratio(timing.n_zq_short_nop))
                | regs::UDDRC_ZQCTL0_T_ZQ_LONG_NOP(self.div_ratio(timing.n_zq_long_nop))
                | regs::UDDRC_ZQCTL0_DIS_SRX_ZQCL
                | regs::UDDRC_ZQCTL0_DIS_AUTO_ZQ,
        ); //(            ZQCTL0)
        self.write_ctrl_reg(
            regs::UDDRC_ZQCTL1,
            regs::UDDRC_ZQCTL1_T_ZQ_RESET_NOP(self.div_ratio(timing.n_zq_reset_nop)),
        ); //(            ZQCTL1)
    }

    fn lpddr4_address_map_set_unused(&self) {
        let layout = self.layout();
        if let Some(rank_width) = layout.rank_width() {
            if rank_width < 2 {
                self.set_ctrl_reg(
                    regs::UDDRC_ADDRMAP0,
                    regs::UDDRC_ADDRMAP0_CS_B1(regs::UDDRC_ADDRMAP_DISABLE),
                )
            }
            if rank_width < 1 {
                self.set_ctrl_reg(
                    regs::UDDRC_ADDRMAP0,
                    regs::UDDRC_ADDRMAP0_CS_B0(regs::UDDRC_ADDRMAP_DISABLE),
                )
            }
        }
        self.set_ctrl_reg(
            regs::UDDRC_ADDRMAP7,
            regs::UDDRC_ADDRMAP7_ADDRMAP_ROW_B17(regs::UDDRC_ADDRMAP_DISABLE),
        ); //(          ADDRMAP7)
        if layout.row_width() < 17 {
            self.set_ctrl_reg(
                regs::UDDRC_ADDRMAP7,
                regs::UDDRC_ADDRMAP7_ADDRMAP_ROW_B16(regs::UDDRC_ADDRMAP_DISABLE),
            ); //(          ADDRMAP7)
        }
        if layout.row_width() < 16 {
            self.set_ctrl_reg(
                regs::UDDRC_ADDRMAP6,
                regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B15(regs::UDDRC_ADDRMAP_DISABLE),
            ); //(          ADDRMAP6)
        }
        if layout.row_width() < 15 {
            self.set_ctrl_reg(
                regs::UDDRC_ADDRMAP6,
                regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B14(regs::UDDRC_ADDRMAP_DISABLE),
            ); //(          ADDRMAP6)
        }

        //col width = 10, hif_col_range[9:0], axi_col_range[10:0]
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP4,
            regs::UDDRC_ADDRMAP4_ADDRMAP_COL_B10(regs::UDDRC_ADDRMAP_DISABLE)
                | regs::UDDRC_ADDRMAP4_ADDRMAP_COL_B11(regs::UDDRC_ADDRMAP_DISABLE),
        ); //(          ADDRMAP4)
    }

    fn lpddr4_address_map_cfg_rank(&self, offset: u32) {
        let layout = self.layout();
        match layout.rank_width() {
            Some(1) => {
                self.write_ctrl_reg(regs::UDDRC_ADDRMAP0, regs::UDDRC_ADDRMAP0_CS_B0(offset))
            }
            Some(2) => self.write_ctrl_reg(
                regs::UDDRC_ADDRMAP0,
                regs::UDDRC_ADDRMAP0_CS_B0(offset) | regs::UDDRC_ADDRMAP0_CS_B1(offset),
            ),
            _ => {}
        }
    }

    fn lpddr4_address_map_cfg_row(&self, offset: u32) {
        let layout = self.layout();
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP5,
            regs::UDDRC_ADDRMAP5_ADDRMAP_ROW_B0(offset)
                | regs::UDDRC_ADDRMAP5_ADDRMAP_ROW_B1(offset)
                | regs::UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10(offset)
                | regs::UDDRC_ADDRMAP5_ADDRMAP_ROW_B11(offset),
        ); //(          ADDRMAP5)
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP6,
            regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B12(offset)
                | regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B13(offset),
        ); //(          ADDRMAP6)
        if layout.row_width() > 14 {
            self.set_ctrl_reg(
                regs::UDDRC_ADDRMAP6,
                regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B14(offset),
            ); //(          ADDRMAP6)
        }
        if layout.row_width() > 15 {
            self.set_ctrl_reg(
                regs::UDDRC_ADDRMAP6,
                regs::UDDRC_ADDRMAP6_ADDRMAP_ROW_B15(offset),
            ); //(          ADDRMAP6)
        }
        if layout.row_width() > 16 {
            self.write_ctrl_reg(
                regs::UDDRC_ADDRMAP7,
                regs::UDDRC_ADDRMAP7_ADDRMAP_ROW_B16(offset),
            ); //(          ADDRMAP7)
        }
    }

    fn lpddr4_address_map_cfg_brc(&self) {
        let layout = self.layout();
        // Note: This direct address map strategy is for simulation/emulation backdoor access
        // For real chip, other more effecient address map stragegy should be taken, such as bank-row switched address map
        //hif_address = rank, bank, row, col
        let rank_offset = layout.ddr_1rank_width() as u32 - 6;
        self.lpddr4_address_map_cfg_rank(rank_offset);
        let bank_offset = layout.ddr_1bank_width() as u32 - 2;
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP1,
            regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B0(bank_offset)
                | regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B1(bank_offset)
                | regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B2(bank_offset),
        ); //(          ADDRMAP1)

        let row_offset = layout.col_width() as u32 - 6;

        self.lpddr4_address_map_cfg_row(row_offset);
        self.lpddr4_address_map_set_unused();
    }

    fn lpddr4_address_map_cfg(&self) {
        let layout = self.layout();
        //config col7-9 to highest， col4-6 to 6-8
        let col7_offset = (layout.ddr_width() - 3) as u32 - 7;
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP3,
            regs::UDDRC_ADDRMAP3_ADDRMAP_COL_B6(2)
                | regs::UDDRC_ADDRMAP3_ADDRMAP_COL_B7(col7_offset)
                | regs::UDDRC_ADDRMAP3_ADDRMAP_COL_B8(col7_offset)
                | regs::UDDRC_ADDRMAP3_ADDRMAP_COL_B9(col7_offset),
        ); //(          ADDRMAP3)

        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP2,
            regs::UDDRC_ADDRMAP2_ADDRMAP_COL_B2(0)
                | regs::UDDRC_ADDRMAP2_ADDRMAP_COL_B3(0)
                | regs::UDDRC_ADDRMAP2_ADDRMAP_COL_B4(2)
                | regs::UDDRC_ADDRMAP2_ADDRMAP_COL_B5(2),
        ); //(          ADDRMAP2)

        //config bank0-1 to 4-5, bank2 to 9
        self.write_ctrl_reg(
            regs::UDDRC_ADDRMAP1,
            regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B0(2)
                | regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B1(2)
                | regs::UDDRC_ADDRMAP1_ADDRMAP_BANK_B2(5),
        ); //(          ADDRMAP1)

        //row from 10
        const ROW_OFFSET: u32 = 10 - 6;
        self.lpddr4_address_map_cfg_row(ROW_OFFSET);

        //rank between 3 msb and rows
        let rank_offset = (layout.ddr_width() - 3 - layout.rank_width().unwrap_or(0)) as u32 - 6;
        self.lpddr4_address_map_cfg_rank(rank_offset);

        self.lpddr4_address_map_set_unused();
    }
}
