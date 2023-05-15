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
        }
    }
}

pub trait Umctl2Lpddr4: Umctrl2MPhyV2 {
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
}
