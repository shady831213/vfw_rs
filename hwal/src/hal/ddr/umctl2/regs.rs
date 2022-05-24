//Modified from https://github.com/linux4sam/at91bootstrap/blob/master/driver/umctl2_regs.h

/*
* Synopsys UMCTL2 register map for multi port configuration.
*/

/* Port status register */
pub const UDDRC_PSTAT:usize = 0x3FC;
/* Port common configuration register */
pub const UDDRC_PCCFG:usize = 0x400;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_0:usize = 0x404;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_0:usize = 0x408;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_0:usize = 0x40C;
/* Port n Control register */
pub const UDDRC_PCTRL_0:usize = 0x490;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_0:usize = 0x494;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_0:usize = 0x498;
/* Port n Write QoS Configuration register 0 */
pub const UDDRC_PCFGWQOS0_0:usize = 0x49C;
/* Port n Write QoS Configuration register 1 */
pub const UDDRC_PCFGWQOS1_0:usize = 0x4A0;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_1:usize = UDDRC_PCFGR_0 + 0xB0;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_1:usize = UDDRC_PCFGW_0 + 0xB0;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_1:usize = UDDRC_PCFGC_0 + 0xB0;
/* Port n Control Register */
pub const UDDRC_PCTRL_1:usize = UDDRC_PCTRL_0 + 0xB0;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_1:usize = UDDRC_PCFGQOS0_0 + 0xB0;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_1:usize = UDDRC_PCFGQOS1_0 + 0xB0;
/* Port n Write QoS Configuration register 0 */
pub const UDDRC_PCFGWQOS0_1:usize = UDDRC_PCFGWQOS0_0 + 0xB0;
/* Port n Write QoS Configuration register 1 */
pub const UDDRC_PCFGWQOS1_1:usize = UDDRC_PCFGWQOS1_0 + 0xB0;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_2:usize = UDDRC_PCFGR_0 + 0xB0 * 2;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_2:usize = UDDRC_PCFGW_0 + 0xB0 * 2;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_2:usize = UDDRC_PCFGC_0 + 0xB0 * 2;
/* Port n Control register */
pub const UDDRC_PCTRL_2:usize = UDDRC_PCTRL_0 + 0xB0 * 2;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_2:usize = UDDRC_PCFGQOS0_0 + 0xB0 * 2;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_2:usize = UDDRC_PCFGQOS1_0 + 0xB0 * 2;
/* Port n Write QoS Confguration register 0 */
pub const UDDRC_PCFGWQOS0_2:usize = UDDRC_PCFGWQOS0_0 + 0xB0 * 2;
/* Port n Write QoS Configuration register 1 */
pub const UDDRC_PCFGWQOS1_2:usize = UDDRC_PCFGWQOS1_0 + 0xB0 * 2;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_3:usize = UDDRC_PCFGR_0 + 0xB0 * 3;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_3:usize = UDDRC_PCFGW_0 + 0xB0 * 3;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_3:usize = UDDRC_PCFGC_0 + 0xB0 * 3;
/* Port n Control Register */
pub const UDDRC_PCTRL_3:usize = UDDRC_PCTRL_0 + 0xB0 * 3;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_3:usize = UDDRC_PCFGQOS0_0 + 0xB0 * 3;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_3:usize = UDDRC_PCFGQOS1_0 + 0xB0 * 3;
/* Port n Write QoS Confguration register 0 */
pub const UDDRC_PCFGWQOS0_3:usize = UDDRC_PCFGWQOS0_0 + 0xB0 * 3;
/* Port n Write QoS Configuration register 1 */
pub const UDDRC_PCFGWQOS1_3:usize = UDDRC_PCFGWQOS1_0 + 0xB0 * 3;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_4:usize = UDDRC_PCFGR_0 + 0xB0 * 4;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_4:usize = UDDRC_PCFGW_0 + 0xB0 * 4;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_4:usize = UDDRC_PCFGC_0 + 0xB0 * 4;
/* Port n Control Register */
pub const UDDRC_PCTRL_4:usize = UDDRC_PCTRL_0 + 0xB0 * 4;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_4:usize = UDDRC_PCFGQOS0_0 + 0xB0 * 4;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_4:usize = UDDRC_PCFGQOS1_0 + 0xB0 * 4;
/* Port n Write QoS Confguration register 0 */
pub const UDDRC_PCFGWQOS0_4:usize = UDDRC_PCFGWQOS0_0 + 0xB0 * 4;
/* Port n Write QoS Configuration register 1 */
pub const UDDRC_PCFGWQOS1_4:usize = UDDRC_PCFGWQOS1_0 + 0xB0 * 4;
/* Port n Configuration Read register */
pub const UDDRC_PCFGR_5:usize = UDDRC_PCFGR_0 + 0xB0 * 5;
/* Port n Configuration Write register */
pub const UDDRC_PCFGW_5:usize = UDDRC_PCFGW_0 + 0xB0 * 5;
/* Port n Configuration Common register */
pub const UDDRC_PCFGC_5:usize = UDDRC_PCFGC_0 + 0xB0 * 5;
/* Port n Control Register */
pub const UDDRC_PCTRL_5:usize = UDDRC_PCTRL_0 + 0xB0 * 5;
/* Port n Read QoS Configuration register 0 */
pub const UDDRC_PCFGQOS0_5:usize = UDDRC_PCFGQOS0_0 + 0xB0 * 5;
/* Port n Read QoS Configuration register 1 */
pub const UDDRC_PCFGQOS1_5:usize = UDDRC_PCFGQOS1_0 + 0xB0 * 5;
/* Port n Write QoS Confguration register 0 */
pub const UDDRC_PCFGWQOS0_5:usize = UDDRC_PCFGWQOS0_0 + 0xB0 * 5;
/* Port n Write QoS Confguration register 1 */
pub const UDDRC_PCFGWQOS1_5:usize = UDDRC_PCFGWQOS1_0 + 0xB0 * 5;
/* SAR Base Address Register 0 */
pub const UDDRC_SARBASE0:usize = 0xF04;
/* SAR Size Register 0 */
pub const UDDRC_SARSIZE0:usize = 0xF08;

/* UMCTL2 MP register helpers */
/* { */
/* -------- UDDRC_PSTAT : (UDDRC_MP Offset: 0x4) Port Status Register -------- */
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 0. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_0:u32 = 0x1u32 << 0;
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 1. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_1:u32 = 0x1u32 << 1;
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 2. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_2:u32 = 0x1u32 << 2;
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 3. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_3:u32 = 0x1u32 << 3;
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 4. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_4:u32 = 0x1u32 << 4;
/* (UDDRC_PSTAT) Indicates if there are outstanding reads for AXI port 5. */
pub const UDDRC_PSTAT_RD_PORT_BUSY_5:u32 = 0x1u32 << 5;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 0. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_0:u32 = 0x1u32 << 16;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 1. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_1:u32 = 0x1u32 << 17;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 2. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_2:u32 = 0x1u32 << 18;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 3. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_3:u32 = 0x1u32 << 19;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 4. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_4:u32 = 0x1u32 << 20;
/* (UDDRC_PSTAT) Indicates if there are outstanding writes for AXI port 5. */
pub const UDDRC_PSTAT_WR_PORT_BUSY_5:u32 = 0x1u32 << 21;

/* -------- UDDRC_PCCFG : (UDDRC_MP Offset: 0x8)
* Port Common Configuration Register --------
*/
/* (UDDRC_PCCFG) If set to 1 (enabled), sets co_gs_go2critical_wr and
* co_gs_go2critical_lpr/co_gs_go2critical_hpr signals going to DDRC based on
* urgent input (awurgent, arurgent) coming from AXI master.
* If set to 0 (disabled), co_gs_go2critical_wr and
* co_gs_go2critical_lpr/co_gs_go2critical_hpr signals at DDRC
* are driven to 1b'0.
*/
pub const UDDRC_PCCFG_GO2CRITICAL_EN:u32 = 0x1u32 << 0;
/* (UDDRC_PCCFG) Page match four limit.
* If set to 1, limits the number of consecutive same page DDRC transactions
* that can be granted by the Port Arbiter to four when Page Match feature
* is enabled.
* If set to 0, there is no limit imposed on number of consecutive
* same page DDRC transactions.
*/
pub const UDDRC_PCCFG_PAGEMATCH_LIMIT:u32 = 0x1u32 << 4;
/* (UDDRC_PCCFG) Burst length expansion mode.
* By default (i.e. bl_exp_mode==0) XPI expands every AXI burst
* into multiple HIF commands, using the memory burst length as a unit.
* If set to 1, then XPI will use half of the memory burst length as a unit.
* This applies to both reads and writes.
* When MSTR.data_bus_width==00, setting bl_exp_mode to 1 has no effect.
* This can be used in cases where Partial Writes is enabled
* (UMCTL2_PARTIAL_WR=1), in order to avoid or minimize t_ccd_l penalty
* in DDR4 and t_ccd_mw penalty in LPDDR4.
* Hence, bl_exp_mode=1 is only recommended if DDR4 or LPDDR4.
*
* Note that if DBICTL.dm_en=0, functionality is not supported in the
* following cases:
*   - UMCTL2_PARTIAL_WR=0
*   - UMCTL2_PARTIAL_WR=1, MSTR.data_bus_width=01,
* MEMC_BURST_LENGTH=8 and MSTR.burst_rdwr=1000 (LPDDR4 only)
*   - UMCTL2_PARTIAL_WR=1, MSTR.data_bus_width=01,
* MEMC_BURST_LENGTH=4 and MSTR.burst_rdwr=0100 (DDR4 only),
* with either MSTR.burstchop=0 or CRCPARCTL1.crc_enable=1
*
* Functionality is also not supported if Data Channel Interleave is enabled
*/
pub const UDDRC_PCCFG_BL_EXP_MODE:u32 = 0x1u32 << 8;
/* -------- UDDRC_PCFGR_0 : (UDDRC_MP Offset: 0xC)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_0) Port priority :
* Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port.
* The aging counters down-count every clock cycle where the port
* is requesting but not granted.
* The higher significant 5-bits of the read aging counter sets the
* priority of the read channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the
* counter starts to decrease.
* When the aging counter becomes 0, the corresponding port channel will have
* the highest priority level (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority
* (2 priority levels: low priority read - LPR, high priority read - HPR) on a
* command by command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_0_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_0_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_0_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_0_RD_PORT_PRIORITY (value:u32) -> u32 {
	UDDRC_PCFGR_0_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_0_RD_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGR_0) bypass read reordering */
pub const UDDRC_PCFGR_0_READ_REORDER_BYPASS_EN:u32 = 0x1u32 << 11;

/* (UDDRC_PCFGR_0) If set to 1, enables aging function for the read channel
* of the port.
*/
pub const UDDRC_PCFGR_0_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/*(UDDRC_PCFGR_0) port urgent enable.
* If set to 1, enables the AXI urgent sideband signal (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr
* signal to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking (it is not associated with
* any particular command).
*/
pub const UDDRC_PCFGR_0_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_0) port match enable.
* If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same memory page
* (same bank and same row). See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_0_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* (UDDRC_PCFGR_0) read write ordered enable */

pub const UDDRC_PCFGR_0_RDWR_ORDERED_EN:u32 = 0x1u32 << 16;

/* -------- UDDRC_PCFGW_0 : (UDDRC_MP Offset: 0x10)
Port n Configuration Write Register --------
*/
pub const UDDRC_PCFGW_0_WR_PORT_PRIORITY_POS:u32 = 0;

/* (UDDRC_PCFGW_0) Write port priority
* Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port.
* The aging counters down-count every clock cycle where the port is requesting
* but not granted.
* The higher significant 5-bits of the write aging counter sets the initial
* priority of the write channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the
* counter starts to decrease. When the aging counter becomes 0, the
* corresponding port channel will have the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set
* port priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when
* they timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_0_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_0_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_0_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_0_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_0_WR_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGW_0) Write port aging enable.
* If set to 1, enables aging function for the
* write channel of the port.
*/
pub const UDDRC_PCFGW_0_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_0) Write port pagematch enable.
* If set to 1, enables the AXI urgent sideband signal (awurgent).
* When enabled and awurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_wr signal to DDRC is asserted
* if enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_0_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_0) Write port pagematch enable.
* If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_0_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCTRL_0 : (UDDRC_MP Offset: 0x98)
* Port n Control Register --------
*/
/* (UDDRC_PCTRL_0) Enables AXI port n. */
pub const UDDRC_PCTRL_0_PORT_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_PCFGQOS0_0 : (UDDRC_MP Offset: 0x9C)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_0) map level 1
* Separation level1 indicating the end of region0 mapping;
* start of region0 is 0.
* Possible values for level1 are 0 to 13 (for dual RAQ) or 0 to 14
* (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_0) map level 2
* Separation level2 indicating the end of region1 mapping.
*/
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGQOS0_0_RQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGQOS0_0) Region 0 qos value.
* This bitfield indicates the traffic class of region 0.
* Valid values are:
* 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are:
* 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class
* of region0 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_0) Region 1 qos value.
* This bitfield indicates the traffic class of region 1.
* Valid values are:
* 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are
* 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class
* of region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION1_POS)
}

/* (UDDRC_PCFGQOS0_0) Region 2 qos value.
* This bitfield indicates the traffic class of region 2.
* Valid values are:
* 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are
* 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class
* of region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2_POS:u32 = 24;
pub const UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2_MSK & ((value) << UDDRC_PCFGQOS0_0_RQOS_MAP_REGION2_POS)
}

/* -------- UDDRC_PCFGQOS1_0 : (UDDRC_MP Offset: 0xA0)
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_0) rqos map timeout blue.
* Specifies the timeout value for transactions mapped to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_0) rqos map timeout red.
* Specifies the timeout value for transactions mapped to the red address queue.
*/
pub const UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_0_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_0 : (UDDRC_MP Offset: 0xA4)
* Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_0) wqos map level 1.
* Separation level indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 14
* which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGWQOS0_0) wqos map level 2.
* Separation level indicating the end of region1 mapping;
* start of region1 is level1 + 1. Possible values for level2 are (leve1+1)
* to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGWQOS0_0_WQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGWQOS0_0) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0)
* and traffic class of region0 is set to 1 (VPW),
* VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_0) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0)
* and traffic class of region 1 is set to 1 (VPW),
* VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_0_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_0 : (UDDRC_MP Offset: 0xA8)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_0) Specifies the timeout value for write transactions.
*/
pub const UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT_MSK & ((value) << UDDRC_PCFGWQOS1_0_WQOS_MAP_TIMEOUT_POS)
}

/* -------- UDDRC_PCFGR_1 : (UDDRC_MP Offset: 0xBC)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_1) Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset,
* or after each grant to the corresponding port.
* The aging counters down-count every clock cycle where the port
* is requesting but not granted.
* The higher significant 5-bits of the read aging counter sets the priority
* of the read channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the counter
* starts to decrease. When the aging counter becomes 0, the corresponding port
* channel will have the highest priority level (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when
* they timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority
* (2 priority levels: low priority read - LPR, high priority read - HPR)
* on a command by command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_1_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_1_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_1_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_1_RD_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGR_1_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_1_RD_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGR_1) bypass read reordering */
pub const UDDRC_PCFGR_1_READ_REORDER_BYPASS_EN:u32 = 0x1u32 << 11;

/* (UDDRC_PCFGR_1) If set to 1, enables aging function for the read channel of the port. */
pub const UDDRC_PCFGR_1_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGR_1) If set to 1, enables the AXI urgent sideband signal
* (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr signal
* to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
*(it is not associated with any particular command).
*/
pub const UDDRC_PCFGR_1_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_1) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_1_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* (UDDRC_PCFGR_0) read write ordered enable */
pub const UDDRC_PCFGR_1_RDWR_ORDERED_EN:u32 = 0x1u32 << 16;

/* -------- UDDRC_PCFGW_1 : (UDDRC_MP Offset: 0xC0)
* Port n Configuration Write Register --------
*/
/* (UDDRC_PCFGW_1) Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset,
* or after each grant to the corresponding port.
* The aging counters down-count every clock cycle where the port is requesting
* but not granted. The higher significant 5-bits of the write aging counter
* sets the initial priority of the write channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the counter
* starts to decrease. When the aging counter becomes 0, the corresponding port
* channel will have the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_1_WR_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGW_1_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_1_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_1_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_1_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_1_WR_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGW_1) If set to 1, enables aging function for the
* write channel of the port.
*/
pub const UDDRC_PCFGW_1_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_1) If set to 1, enables the AXI urgent sideband signal
* (awurgent). When enabled and awurgent is asserted by the master,
* that port becomes the highest priority and co_gs_go2critical_wr signal to
* DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be  asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_1_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_1) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_1_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCTRL_1 : (UDDRC_MP Offset: 0x148)
* Port n Control Register --------
*/
/* (UDDRC_PCTRL_1) Enables AXI port n.
*/
pub const UDDRC_PCTRL_1_PORT_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_PCFGQOS0_1 : (UDDRC_MP Offset: 0x14C)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_1) Separation level1 indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 13 (for dual RAQ)
* or 0 to 14 (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_1) Separation level2 indicating the end of region1 mapping;
*/
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGQOS0_1_RQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGQOS0_1) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are: 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0
* is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_1) This bitfield indicates the traffic class of region 1.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class
* of region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION1_POS)
}

/* (UDDRC_PCFGQOS0_1) This bitfield indicates the traffic class of region 2.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region2 maps to the red? address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class
* of region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2_POS:u32 = 24;
pub const UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2_MSK & ((value) << UDDRC_PCFGQOS0_1_RQOS_MAP_REGION2_POS)
}

/* -------- UDDRC_PCFGQOS1_1 : (UDDRC_MP Offset: 0x150)
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_1) Specifies the timeout value for transactions
* mapped to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_1) Specifies the timeout value for transactions
* mapped to the red address queue.
*/
pub const UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_1_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_1 : (UDDRC_MP Offset: 0x154)
* Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_1) Separation level indicating the end of region0 mapping;
* start of region0 is 0.
* Possible values for level1 are 0 to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGWQOS0_1) Separation level indicating the end of region1 mapping;
* start of region1 is level1 + 1.
* Possible values for level2 are (leve1+1) to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGWQOS0_1_WQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGWQOS0_1) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_1) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region 1
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_1_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_1 : (UDDRC_MP Offset: 0x158)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_1) Specifies the timeout value for write transactions.
*/
pub const UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT_MSK & ((value) << UDDRC_PCFGWQOS1_1_WQOS_MAP_TIMEOUT_POS)
}

/* -------- UDDRC_PCFGR_2 : (UDDRC_MP Offset: 0x16C)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_2) Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset,
* or after each grant to the corresponding port.
* The aging counters down-count every clock cycle where the port is requesting
* but not granted. The higher significant 5-bits of the read aging counter
* sets the priority of the read channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the counter
* starts to decrease. When the aging counter becomes 0, the corresponding port
* channel will have the highest priority level (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority
* (2 priority levels: low priority read - LPR, high priority read - HPR)
* on a command by command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_2_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_2_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_2_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_2_RD_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGR_2_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_2_RD_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGR_2) bypass read reordering */
pub const UDDRC_PCFGR_2_READ_REORDER_BYPASS_EN:u32 = 0x1u32 << 11;

/* (UDDRC_PCFGR_2) If set to 1, enables aging function for the read channel
* of the port.
*/
pub const UDDRC_PCFGR_2_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/*(UDDRC_PCFGR_2) If set to 1, enables the AXI urgent sideband signal (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr
* signal to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGR_2_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_2) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_2_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCFGW_2 : (UDDRC_MP Offset: 0x170)
* Port n Configuration Write Register --------
*/
/* (UDDRC_PCFGR_2) read write ordered enable */
pub const UDDRC_PCFGR_2_RDWR_ORDERED_EN:u32 = 0x1u32 << 16;

/*(UDDRC_PCFGW_2) Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not granted.
* The higher significant 5-bits of the write aging counter sets the initial
* priority of the write channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the counter
* starts to decrease. When the aging counter becomes 0, the corresponding port
* channel will have the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_2_WR_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGW_2_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_2_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_2_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_2_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_2_WR_PORT_PRIORITY_POS)
}

/*(UDDRC_PCFGW_2) If set to 1, enables aging function for the write channel
* of the port.
*/
pub const UDDRC_PCFGW_2_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_2) If set to 1, enables the AXI urgent sideband signal (awurgent).
* When enabled and awurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_wr signal to DDRC is asserted
* if enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_2_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_2) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same memory page
* (same bank and same row). See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_2_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCTRL_2 : (UDDRC_MP Offset: 0x1F8)
* Port n Control Register --------
*/
/* (UDDRC_PCTRL_2) Enables AXI port n. */
pub const UDDRC_PCTRL_2_PORT_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_PCFGQOS0_2 : (UDDRC_MP Offset: 0x1FC)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_2) Separation level1 indicating the end of region0 mapping;
* start of region0 is 0.
* Possible values for level1 are 0 to 13 (for dual RAQ)
* or 0 to 14 (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_2) Separation level2 indicating the end of region1 mapping;
* start of region1 is (level1 + 1).
* Possible values for level2 are (level1 + 1) to 14 which corresponds to arqos.
* Region2 starts from (level2 + 1) up to 15.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGQOS0_2_RQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGQOS0_2) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are: 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0
* is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_2) This bitfield indicates the traffic class of region 1.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region 1
* is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION1_POS)
}

/* (UDDRC_PCFGQOS0_2) This bitfield indicates the traffic class of region2.
* For dual address queue configurations, region2 maps to the red address queue.
* Valid values are 1: VPR and 2: HPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region2
* is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2_POS:u32 = 24;
pub const UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2_MSK & ((value) << UDDRC_PCFGQOS0_2_RQOS_MAP_REGION2_POS)
}
/* -------- UDDRC_PCFGQOS1_2 : (UDDRC_MP Offset: 0x200)
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_2) Specifies the timeout value for transactions
* mapped to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_2) Specifies the timeout value for transactions
* mapped to the red address queue.
*/
pub const UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_2_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_2 : (UDDRC_MP Offset: 0x204)
* Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_2) Separation level indicating the end of region0 mapping;
* start of region0 is 0.
* Possible values for level1 are 0 to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGWQOS0_1) Separation level indicating the end of region1 mapping;
* start of region1 is level1 + 1.
* Possible values for level2 are (leve1+1) to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGWQOS0_2_WQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGWQOS0_2) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_2) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region 1
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_2_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_2 : (UDDRC_MP Offset: 0x208)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_2) Specifies the timeout value for write transactions. */
pub const UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1_MSK & ((value) << UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT1_POS)
}

/* (UDDRC_PCFGWQOS1_2) Specifies the timeout value for write transactions. */
pub const UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2_POS:u32 = 16;
pub const UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2_MSK & ((value) << UDDRC_PCFGWQOS1_2_WQOS_MAP_TIMEOUT2_POS)
}

/* -------- UDDRC_PCFGR_3 : (UDDRC_MP Offset: 0x21C)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_3) Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port.
* The aging counters down-count every clock cycle where the port is requesting
* but not granted. The higher significant 5-bits of the read aging counter
* sets the priority of the read channel of a given port.
* Port's priority will increase as the higher significant 5-bits of the counter
* starts to decrease. When the aging counter becomes 0, the corresponding port
* channel will have the highest priority level (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority (2 priority levels:
* low priority read - LPR, high priority read - HPR)
* on a command by command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_3_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_3_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_3_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_3_RD_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGR_3_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_3_RD_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGR_3) bypass read reordering */
pub const UDDRC_PCFGR_3_READ_REORDER_BYPASS_EN:u32 = 0x1u32 << 11;

/* (UDDRC_PCFGR_3) If set to 1, enables aging function for the read
* channel of the port.
*/
pub const UDDRC_PCFGR_3_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGR_3) If set to 1, enables the AXI urgent sideband signal (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr
* signal to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGR_3_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_3) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same memory page
* (same bank and same row). See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_3_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCFGW_3 : (UDDRC_MP Offset: 0x220)
* Port n Configuration Write Register --------
*/
/* (UDDRC_PCFGR_3) read write ordered enable */
pub const UDDRC_PCFGR_3_RDWR_ORDERED_EN:u32 = 0x1u32 << 16;

/* (UDDRC_PCFGW_3) Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not granted.
* The higher significant 5-bits of the write aging counter sets the initial
* priority of the write channel of a given port. Port's priority will increase
* as the higher significant 5-bits of the counter starts to decrease.
* When the aging counter becomes 0, the corresponding port channel will have
* the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_3_WR_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGW_3_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_3_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_3_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_3_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_3_WR_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGW_3) If set to 1, enables aging function for the write
* channel of the port.
*/
pub const UDDRC_PCFGW_3_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_3) If set to 1, enables the AXI urgent sideband signal (awurgent).
* When enabled and awurgent is asserted by the master, that port becomes the
* highest priority and co_gs_go2critical_wr signal to DDRC is asserted if
* enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_3_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_3) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_3_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCTRL_3 : (UDDRC_MP Offset: 0x2A8)
* Port n Control Register --------
*/

/* (UDDRC_PCTRL_3) Enables AXI port n. */
pub const UDDRC_PCTRL_3_PORT_EN:u32 = 0x1u32 << 0;
/* -------- UDDRC_PCFGQOS0_3 : (UDDRC_MP Offset: 0x2AC)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_3) Separation level1 indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 13 (for dual RAQ)
* or 0 to 14 (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_3) Separation level2 indicating the end of region1 mapping;
* start of region1 is (level1 + 1). Possible values for level2 are (level1 + 1)
* to 14 which corresponds to arqos.
* Region2 starts from (level2 + 1) up to 15.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGQOS0_3_RQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGQOS0_3) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are: 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region0 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_3) This bitfield indicates the traffic class of region 1.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION1_POS)
}

/* (UDDRC_PCFGQOS0_3) This bitfield indicates the traffic class of region2.
* For dual address queue configurations, region2 maps to the red address queue.
* Valid values are 1: VPR and 2: HPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region2 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2_POS:u32 = 24;
pub const UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2_MSK & ((value) << UDDRC_PCFGQOS0_3_RQOS_MAP_REGION2_POS)
}

/* -------- UDDRC_PCFGQOS1_3 : (UDDRC_MP Offset: 0x2B0)
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_3) Specifies the timeout value for transactions mapped
* to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_3) Specifies the timeout value for transactions mapped
* to the red address queue.
*/
pub const UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_3_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_3 : (UDDRC_MP Offset: 0x2B4)
Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_3) Separation level indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 14 which
* corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGWQOS0_3) Separation level indicating the end of region1 mapping;
* start of region1 is level1 + 1. Possible values for level2 are (leve1+1)
* to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGWQOS0_3_WQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGWQOS0_3) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of
* region0 is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_3) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of
* region 1 is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_3_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_3 : (UDDRC_MP Offset: 0x2B8)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_3) Specifies the timeout value for write transactions.
*/
pub const UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT_MSK & ((value) << UDDRC_PCFGWQOS1_3_WQOS_MAP_TIMEOUT_POS)
}

/* -------- UDDRC_PCFGR_4 : (UDDRC_MP Offset: 0x2CC)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_4) Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not	granted.
* The higher significant 5-bits of the read aging counter sets the priority
* of the read channel of a given port.	 Port's priority will increase as the
* higher significant 5-bits of the counter starts to decrease.
* When the aging counter becomes 0, the corresponding port channel will have
* the highest priority level (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority  (2 priority levels:
* low priority read - LPR, high priority read - HPR) on a command by
* command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_4_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_4_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_4_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_4_RD_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGR_4_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_4_RD_PORT_PRIORITY_POS)
}


/* (UDDRC_PCFGR_4) bypass read reordering */
pub const UDDRC_PCFGR_4_READ_REORDER_BYPASS_EN:u32 = 0x1u32 << 11;

/* (UDDRC_PCFGR_4) If set to 1, enables aging function for the read channel
* of the port.
*/
pub const UDDRC_PCFGR_4_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGR_4) If set to 1, enables the AXI urgent sideband signal (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr signal
* to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGR_4_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_4) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_4_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCFGW_4 : (UDDRC_MP Offset: 0x2D0)
* Port n Configuration Write Register --------
*/
/* (UDDRC_PCFGR_4) read write ordered enable */
pub const UDDRC_PCFGR_4_RDWR_ORDERED_EN:u32 = 0x1u32 << 16;

/* (UDDRC_PCFGW_4) Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not granted. The higher significant 5-bits
* of the write aging counter sets the initial priority of the write channel
* of a given port. Port's priority will increase as the higher significant
* 5-bits of the counter starts to decrease. When the aging counter becomes 0,
* the corresponding port channel will have the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_4_WR_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGW_4_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_4_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_4_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_4_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_4_WR_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGW_4) If set to 1, enables aging function for the write
* channel of the port.
*/
pub const UDDRC_PCFGW_4_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_4) If set to 1, enables the AXI urgent sideband signal (awurgent).
* When enabled and awurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_wr signal to DDRC is asserted
* if enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_4_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_4) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued
* to be granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_4_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCTRL_4 : (UDDRC_MP Offset: 0x358)
* Port n Control Register --------
*/
/* (UDDRC_PCTRL_4) Enables AXI port n. */
pub const UDDRC_PCTRL_4_PORT_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_PCFGQOS0_4 : (UDDRC_MP Offset: 0x35C)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_4) Separation level1 indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 13 (for dual RAQ)
* or 0 to 14 (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_4) Separation level1 indicating the end of region1 mapping;
*/
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGQOS0_4_RQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGQOS0_4) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are: 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0
* is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_4) This bitfield indicates the traffic class of region 1.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION1_POS)
}

/* (UDDRC_PCFGQOS0_4) This bitfield indicates the traffic class of region 2.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region2 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2_POS:u32 = 24;
pub const UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2_MSK & ((value) << UDDRC_PCFGQOS0_4_RQOS_MAP_REGION2_POS)
}

/* -------- UDDRC_PCFGQOS1_4 : (UDDRC_MP Offset: 0x360) \
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_4) Specifies the timeout value for transactions mapped
* to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_4) Specifies the timeout value for transactions mapped
* to the red address queue.
*/
pub const UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_4_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_4 : (UDDRC_MP Offset: 0x364)
* Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_4) Separation level indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 14
* which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGWQOS0_4) Separation level indicating the end of region1 mapping;
* start of region1 is level1 + 1. Possible values for level2 are (leve1+1)
* to 14 which corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2_POS:u32 = 8;
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2_MSK & ((value) << UDDRC_PCFGWQOS0_4_WQOS_MAP_LEVEL2_POS)
}

/* (UDDRC_PCFGWQOS0_4) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_4) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of
* region 1 is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_4_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_4 : (UDDRC_MP Offset: 0x368)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_4) Specifies the timeout value for write transactions.
*/
pub const UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT_MSK & ((value) << UDDRC_PCFGWQOS1_4_WQOS_MAP_TIMEOUT_POS)
}

/* -------- UDDRC_PCFGR_5 : (UDDRC_MP Offset: 0x37C)
* Port n Configuration Read Register --------
*/
/* (UDDRC_PCFGR_5) Determines the initial load value of read aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not granted. The higher significant 5-bits
* of the read aging counter sets the priority of the read channel of
* a given port. Port's priority will increase as the higher significant 5-bits
* of the counter starts to decrease. When the aging counter becomes 0,
* the corresponding port channel will have the highest priority level
* (timeout condition - Priority0).
* For multi-port configurations, the aging counters cannot be used to set
* port priorities when external dynamic priority inputs (arqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* In this case, external dynamic priority input, arqos (for reads only)
* can still be used to set the DDRC read priority (2 priority levels:
* low priority read - LPR, high priority read - HPR)
* on a command by command basis.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGR_5_RD_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGR_5_RD_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGR_5_RD_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGR_5_RD_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGR_5_RD_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGR_5_RD_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGR_5) If set to 1, enables aging function for the read channel
* of the port.
*/
pub const UDDRC_PCFGR_5_RD_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGR_5) If set to 1, enables the AXI urgent sideband signal (arurgent).
* When enabled and arurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_lpr/co_gs_go2critical_hpr signal
* to DDRC is asserted if enabled in PCCFG.go2critical_en register.
* Note that arurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGR_5_RD_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGR_5) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same
* memory page (same bank and same row).
* See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGR_5_RD_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;

/* -------- UDDRC_PCFGW_5 : (UDDRC_MP Offset: 0x380)
* Port n Configuration Write Register --------
*/
/* (UDDRC_PCFGW_5) Determines the initial load value of write aging counters.
* These counters will be parallel loaded after reset, or after each grant
* to the corresponding port. The aging counters down-count every clock cycle
* where the port is requesting but not granted. The higher significant 5-bits
* of the write aging counter sets the initial priority of the write channel
* of a given port. Port's priority will increase as the higher significant
* 5-bits of the counter starts to decrease. When the aging counter becomes 0,
* the corresponding port channel will have the highest priority level.
* For multi-port configurations, the aging counters cannot be used to set port
* priorities when external dynamic priority inputs (awqos) are enabled
* (timeout is still applicable).
* For single port configurations, the aging counters are only used when they
* timeout (become 0) to force read-write direction switching.
* Note: The two LSBs of this register field are tied internally to 2'b00.
*/
pub const UDDRC_PCFGW_5_WR_PORT_PRIORITY_POS:u32 = 0;
pub const UDDRC_PCFGW_5_WR_PORT_PRIORITY_MSK:u32 = 0x3ffu32 << UDDRC_PCFGW_5_WR_PORT_PRIORITY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGW_5_WR_PORT_PRIORITY (value:u32) -> u32 {
    UDDRC_PCFGW_5_WR_PORT_PRIORITY_MSK & ((value) << UDDRC_PCFGW_5_WR_PORT_PRIORITY_POS)
}

/* (UDDRC_PCFGW_5) If set to 1, enables aging function for the write channel
* of the port.
*/
pub const UDDRC_PCFGW_5_WR_PORT_AGING_EN:u32 = 0x1u32 << 12;

/* (UDDRC_PCFGW_5) If set to 1, enables the AXI urgent sideband signal (awurgent).
* When enabled and awurgent is asserted by the master, that port becomes
* the highest priority and co_gs_go2critical_wr signal to DDRC is asserted
* if enabled in PCCFG.go2critical_en register.
* Note that awurgent signal can be asserted anytime and as long as required
* which is independent of address handshaking
* (it is not associated with any particular command).
*/
pub const UDDRC_PCFGW_5_WR_PORT_URGENT_EN:u32 = 0x1u32 << 13;

/* (UDDRC_PCFGW_5) If set to 1, enables the Page Match feature.
* If enabled, once a requesting port is granted, the port is continued to be
* granted if the following immediate commands are to the same memory page
* (same bank and same row). See also related PCCFG.pagematch_limit register.
*/
pub const UDDRC_PCFGW_5_WR_PORT_PAGEMATCH_EN:u32 = 0x1u32 << 14;
/* -------- UDDRC_PCTRL_5 : (UDDRC_MP Offset: 0x408)
Port n Control Register --------
*/
/* (UDDRC_PCTRL_5) Enables AXI port n. */
pub const UDDRC_PCTRL_5_PORT_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_PCFGQOS0_5 : (UDDRC_MP Offset: 0x40C)
* Port n Read QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGQOS0_5) Separation level1 indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 13 (for dual RAQ)
* or 0 to 14 (for single RAQ) which corresponds to arqos.
* Note that for PA, arqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
* All of the map_level* registers must be set to distinct values.
*/
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1_POS:u32 = 0;
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1_MSK:u32 = 0xfu32 << UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1_MSK & ((value) << UDDRC_PCFGQOS0_5_RQOS_MAP_LEVEL1_POS)
}

/* (UDDRC_PCFGQOS0_5) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region 0 maps to the blue address queue.
* In this case, valid values are: 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region0 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGQOS0_5_RQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGQOS0_5) This bitfield indicates the traffic class of region 1.
* Valid values are: 0 : LPR, 1: VPR, 2: HPR.
* For dual address queue configurations, region1 maps to the blue address queue.
* In this case, valid values are 0: LPR and 1: VPR only.
* When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of
* region 1 is set to 1 (VPR), VPR traffic is aliased to LPR traffic.
*/
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGQOS0_5_RQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGQOS1_5 : (UDDRC_MP Offset: 0x410)
* Port n Read QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGQOS1_5) Specifies the timeout value for transactions mapped
* to the blue address queue.
*/
pub const UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB_POS:u32 = 0;
pub const UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB (value:u32) -> u32 {
    UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB_MSK & ((value) << UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTB_POS)
}

/* (UDDRC_PCFGQOS1_5) Specifies the timeout value for transactions mapped
* to the red address queue.
*/
pub const UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR_POS:u32 = 16;
pub const UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR_MSK:u32 = 0x7ffu32 << UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR (value:u32) -> u32 {
    UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR_MSK & ((value) << UDDRC_PCFGQOS1_5_RQOS_MAP_TIMEOUTR_POS)
}

/* -------- UDDRC_PCFGWQOS0_5 : (UDDRC_MP Offset: 0x414)
* Port n Write QoS Configuration Register 0 --------
*/
/* (UDDRC_PCFGWQOS0_5) Separation level indicating the end of region0 mapping;
* start of region0 is 0. Possible values for level1 are 0 to 14 which
* corresponds to awqos.
* Note that for PA, awqos values are used directly as port priorities,
* where the higher the value corresponds to higher port priority.
*/
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL_POS:u32 = 0;
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL_MSK:u32 = 0xfu32 << UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL_MSK & ((value) << UDDRC_PCFGWQOS0_5_WQOS_MAP_LEVEL_POS)
}

/* (UDDRC_PCFGWQOS0_5) This bitfield indicates the traffic class of region 0.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0
* is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0_POS:u32 = 16;
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0_MSK & ((value) << UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION0_POS)
}

/* (UDDRC_PCFGWQOS0_5) This bitfield indicates the traffic class of region 1.
* Valid values are: 0: NPW, 1: VPW.
* When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of
* region 1 is set to 1 (VPW), VPW traffic is aliased to NPW traffic.
*/
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1_POS:u32 = 20;
pub const UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1_MSK:u32 = 0x3u32 << UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1 (value:u32) -> u32 {
    UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1_MSK & ((value) << UDDRC_PCFGWQOS0_5_WQOS_MAP_REGION1_POS)
}

/* -------- UDDRC_PCFGWQOS1_5 : (UDDRC_MP Offset: 0x418)
* Port n Write QoS Configuration Register 1 --------
*/
/* (UDDRC_PCFGWQOS1_5) Specifies the timeout value for write transactions.*/
pub const UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT_POS:u32 = 0;
pub const UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT_MSK:u32 = 0x7ffu32 << UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT (value:u32) -> u32 {
    UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT_MSK & ((value) << UDDRC_PCFGWQOS1_5_WQOS_MAP_TIMEOUT_POS)
}

/* } */
/* End of UMCTL2 MP register helpers */

/*
* Synopsys UMCTL2 register map for main configuration.
*/
/* (UDDRC_REGS Offset: 0x0) Master Register0 */
pub const UDDRC_MSTR:usize = 0x0;
/* (UDDRC_REGS Offset: 0x4) Operating Mode Status Register */
pub const UDDRC_STAT:usize = 0x4;
/* (UDDRC_REGS Offset: 0x10) Mode Register Read/Write Control Register 0. */
pub const UDDRC_MRCTRL0:usize = 0x10;
/* (UDDRC_REGS Offset: 0x14) Mode Register Read/Write Control Register 1 */
pub const UDDRC_MRCTRL1:usize = 0x14;
/* (UDDRC_REGS Offset: 0x18) Mode Register Read/Write Status Register */
pub const UDDRC_MRSTAT:usize = 0x18;
/* (UDDRC_REGS Offset: 0x1C Mode Register Read/Write Control Register 2 */
pub const UDDRC_MRCTRL2:usize = 0x1C;
/* (UDDRC_REGS Offset: 0x20) Temperature Derate Enable Register */
pub const UDDRC_DERATEEN:usize = 0x20;
/* (UDDRC_REGS Offset: 0x24) Temperature Derate Interval Register */
pub const UDDRC_DERATEINT:usize = 0x24;
/* (UDDRC_REGS Offset: 0x30) Low Power Control Register */
pub const UDDRC_PWRCTL:usize = 0x30;
/* (UDDRC_REGS Offset: 0x34) Low Power Timing Register */
pub const UDDRC_PWRTMG:usize = 0x34;
/* (UDDRC_REGS Offset: 0x38) Hardware Low Power Control Register */
pub const UDDRC_HWLPCTL:usize = 0x38;
/* (UDDRC_REGS Offset: 0x50) Refresh Control Register 0 */
pub const UDDRC_RFSHCTL0:usize = 0x50;
/* (UDDRC_REGS Offset: 0x60) Refresh Control Register 3 */
pub const UDDRC_RFSHCTL3:usize = 0x60;
/* (UDDRC_REGS Offset: 0x64) Refresh Timing Register */
pub const UDDRC_RFSHTMG:usize = 0x64;
/* (UDDRC_REGS Offset: 0x68) Refresh Timing Register 1*/
pub const UDDRC_RFSHTMG1:usize = 0x68;
/* (UDDRC_REGS Offset: 0xC0) CRC Parity Control Register0 */
pub const UDDRC_CRCPARCTL0:usize = 0xC0;
/* (UDDRC_REGS Offset: 0xC4) CRC Parity Control Register1 */
pub const UDDRC_CRCPARCTL1:usize = 0xC4;
/* (UDDRC_REGS Offset: 0xC8) CRC Parity Control Register2 */
pub const UDDRC_CRCPARCTL2:usize = 0xC8;
/* (UDDRC_REGS Offset: 0xCC) CRC Parity Status Register */
pub const UDDRC_CRCPARSTAT:usize = 0xCC;
/* (UDDRC_REGS Offset: 0xD0) SDRAM Initialization Register 0 */
pub const UDDRC_INIT0:usize = 0xD0;
/* (UDDRC_REGS Offset: 0xD4) SDRAM Initialization Register 1 */
pub const UDDRC_INIT1:usize = 0xD4;
/* (UDDRC_REGS Offset: 0xD8) SDRAM Initialization Register 2 */
pub const UDDRC_INIT2:usize = 0xD8;
/* (UDDRC_REGS Offset: 0xDC) SDRAM Initialization Register 3 */
pub const UDDRC_INIT3:usize = 0xDC;
/* (UDDRC_REGS Offset: 0xE0) SDRAM Initialization Register 4 */
pub const UDDRC_INIT4:usize = 0xE0;
/* (UDDRC_REGS Offset: 0xE4) SDRAM Initialization Register 5 */
pub const UDDRC_INIT5:usize = 0xE4;
/* (UDDRC_REGS Offset: 0xE8) SDRAM Initialization Register 6 */
pub const UDDRC_INIT6:usize = 0xE8;
/* (UDDRC_REGS Offset: 0xEC) SDRAM Initialization Register 7 */
pub const UDDRC_INIT7:usize = 0xEC;
/* (UDDRC_REGS Offset: 0xF0) DIMM Control Register */
pub const UDDRC_DIMMCTL:usize = 0xF0;
/* (UDDRC_REGS Offset: 0x100) SDRAM Timing Register 0 */
pub const UDDRC_DRAMTMG0:usize = 0x100;
/* (UDDRC_REGS Offset: 0x104) SDRAM Timing Register 1 */
pub const UDDRC_DRAMTMG1:usize = 0x104;
/* (UDDRC_REGS Offset: 0x108) SDRAM Timing Register 2 */
pub const UDDRC_DRAMTMG2:usize = 0x108;
/* (UDDRC_REGS Offset: 0x10C) SDRAM Timing Register 3 */
pub const UDDRC_DRAMTMG3:usize = 0x10C;
/* (UDDRC_REGS Offset: 0x110) SDRAM Timing Register 4 */
pub const UDDRC_DRAMTMG4:usize = 0x110;
/* (UDDRC_REGS Offset: 0x114) SDRAM Timing Register 5 */
pub const UDDRC_DRAMTMG5:usize = 0x114;
/* (UDDRC_REGS Offset: 0x118) SDRAM Timing Register 6 */
pub const UDDRC_DRAMTMG6:usize = 0x118;
/* (UDDRC_REGS Offset: 0x11C) SDRAM Timing Register 7 */
pub const UDDRC_DRAMTMG7:usize = 0x11C;
/* (UDDRC_REGS Offset: 0x120) SDRAM Timing Register 8 */
pub const UDDRC_DRAMTMG8:usize = 0x120;
/* (UDDRC_REGS Offset: 0x124) SDRAM Timing Register 9 */
pub const UDDRC_DRAMTMG9:usize = 0x124;
/* (UDDRC_REGS Offset: 0x128) SDRAM Timing Register 10 */
pub const UDDRC_DRAMTMG10:usize = 0x128;
/* (UDDRC_REGS Offset: 0x12C) SDRAM Timing Register 11 */
pub const UDDRC_DRAMTMG11:usize = 0x12C;
/* (UDDRC_REGS Offset: 0x130) SDRAM Timing Register 12 */
pub const UDDRC_DRAMTMG12:usize = 0x130;
/* (UDDRC_REGS Offset: 0x134) SDRAM Timing Register 13 */
pub const UDDRC_DRAMTMG13:usize = 0x134;
/* (UDDRC_REGS Offset: 0x138) SDRAM Timing Register 14 */
pub const UDDRC_DRAMTMG14:usize = 0x138;
/* (UDDRC_REGS Offset: 0x13C) SDRAM Timing Register 15 */
pub const UDDRC_DRAMTMG15:usize = 0x13C;
/* (UDDRC_REGS Offset: 0x180) ZQ Control Register 0 */
pub const UDDRC_ZQCTL0:usize = 0x180;
/* (UDDRC_REGS Offset: 0x184) ZQ Control Register 1 */
pub const UDDRC_ZQCTL1:usize = 0x184;
/* (UDDRC_REGS Offset: 0x188) ZQ Control Register 2 */
pub const UDDRC_ZQCTL2:usize = 0x188;
/* (UDDRC_REGS Offset: 0x18C) ZQ Status Register */
pub const UDDRC_ZQSTAT:usize = 0x18C;
/* (UDDRC_REGS Offset: 0x190) DFI Timing Register 0 */
pub const UDDRC_DFITMG0:usize = 0x190;
/* (UDDRC_REGS Offset: 0x194) DFI Timing Register 1 */
pub const UDDRC_DFITMG1:usize = 0x194;
/* (UDDRC_REGS Offset: 0x198) DFI Low Power Configuration Register 0 */
pub const UDDRC_DFILPCFG0:usize = 0x198;
/* (UDDRC_REGS Offset: 0x19C) DFI Low Power Configuration Register 1 */
pub const UDDRC_DFILPCFG1:usize = 0x19C;
/* (UDDRC_REGS Offset: 0x1A0) DFI Update Register 0 */
pub const UDDRC_DFIUPD0:usize = 0x1A0;
/* (UDDRC_REGS Offset: 0x1A4) DFI Update Register 1 */
pub const UDDRC_DFIUPD1:usize = 0x1A4;
/* (UDDRC_REGS Offset: 0x1A8) DFI Update Register 2 */
pub const UDDRC_DFIUPD2:usize = 0x1A8;
/* (UDDRC_REGS Offset: 0x1B0) DFI Miscellaneous Control Register */
pub const UDDRC_DFIMISC:usize = 0x1B0;
/* (UDDRC_REGS Offset: 0x1B4) DFI Timing Register 2 */
pub const UDDRC_DFITMG2:usize = 0x1B4;
/* (UDDRC_REGS Offset: 0x1B8) DFI Timing Register 3 */
pub const UDDRC_DFITMG3:usize = 0x1B8;
/* (UDDRC_REGS Offset: 0x1BC) DFI Status Register */
pub const UDDRC_DFISTAT:usize = 0x1BC;
/* (UDDRC_REGS Offset: 0x1C0) DBI Control Register */
pub const UDDRC_DBICTL:usize = 0x1C0;
/* (UDDRC_REGS Offset: 0x1C4) DFI PHY Master */
pub const UDDRC_DFIPHYMSTR:usize = 0x1C4;
/* (UDDRC_REGS Offset: 0x204) Address Map Register 1 */
pub const UDDRC_ADDRMAP1:usize = 0x204;
/* (UDDRC_REGS Offset: 0x208) Address Map Register 2 */
pub const UDDRC_ADDRMAP2:usize = 0x208;
/* (UDDRC_REGS Offset: 0x20C) Address Map Register 3 */
pub const UDDRC_ADDRMAP3:usize = 0x20C;
/* (UDDRC_REGS Offset: 0x210) Address Map Register 4 */
pub const UDDRC_ADDRMAP4:usize = 0x210;
/* (UDDRC_REGS Offset: 0x214) Address Map Register 5 */
pub const UDDRC_ADDRMAP5:usize = 0x214;
/* (UDDRC_REGS Offset: 0x218) Address Map Register 6 */
pub const UDDRC_ADDRMAP6:usize = 0x218;
/* (UDDRC_REGS Offset: 0x21C) Address Map Register 7 */
pub const UDDRC_ADDRMAP7:usize = 0x21C;
/* (UDDRC_REGS Offset: 0x220) Address Map Register 8 */
pub const UDDRC_ADDRMAP8:usize = 0x220;
/* (UDDRC_REGS Offset: 0x224) Address Map Register 9 */
pub const UDDRC_ADDRMAP9:usize = 0x224;
/* (UDDRC_REGS Offset: 0x228) Address Map Register 10 */
pub const UDDRC_ADDRMAP10:usize = 0x228;
/* (UDDRC_REGS Offset: 0x22C) Address Map Register 11 */
pub const UDDRC_ADDRMAP11:usize = 0x22C;
/* (UDDRC_REGS Offset: 0x240) ODT Configuration Register */
pub const UDDRC_ODTCFG:usize = 0x240;
/* (UDDRC_REGS Offset: 0x244) ODT/Rank Map Register */
pub const UDDRC_ODTMAP:usize = 0x244;
/* (UDDRC_REGS Offset: 0x250) Scheduler Control Register */
pub const UDDRC_SCHED:usize = 0x250;
/* (UDDRC_REGS Offset: 0x254) Scheduler Control Register 1 */
pub const UDDRC_SCHED1:usize = 0x254;
/* (UDDRC_REGS Offset: 0x25C) High Priority Read CAM Register 1 */
pub const UDDRC_PERFHPR1:usize = 0x25C;
/* (UDDRC_REGS Offset: 0x264) Low Priority Read CAM Register 1 */
pub const UDDRC_PERFLPR1:usize = 0x264;
/* (UDDRC_REGS Offset: 0x26C) Write CAM Register 1 */
pub const UDDRC_PERFWR1:usize = 0x26C;
/* (UDDRC_REGS Offset: 0x300) Debug Register 0 */
pub const UDDRC_DBG0:usize = 0x300;
/* (UDDRC_REGS Offset: 0x304) Debug Register 1 */
pub const UDDRC_DBG1:usize = 0x304;
/* (UDDRC_REGS Offset: 0x308) CAM Debug Register */
pub const UDDRC_DBGCAM:usize = 0x308;
/* (UDDRC_REGS Offset: 0x30C) Command Debug Register */
pub const UDDRC_DBGCMD:usize = 0x30C;
/* (UDDRC_REGS Offset: 0x310) Status Debug Register */
pub const UDDRC_DBGSTAT:usize = 0x310;
/* (UDDRC_REGS Offset: 0x320) Software Register Programming Control Enable */
pub const UDDRC_SWCTL:usize = 0x320;
/* (UDDRC_REGS Offset: 0x324) Software Register Programming Control Status */
pub const UDDRC_SWSTAT:usize = 0x324;
/* (UDDRC_REGS Offset: 0x36C) AXI Poison Configuration Register.
* Common for all AXI ports */
pub const UDDRC_POISONCFG:usize = 0x36C;
/* (UDDRC_REGS Offset: 0x370) AXI Poison Status Register */
pub const UDDRC_POISONSTAT:usize = 0x370;


/* UMCTL2 main register helpers */
/* { */

/* -------- UDDRC_MSTR : (UDDRC_REGS Offset: 0x0) Master Register0 -------- */
/* (UDDRC_MSTR) Select DDR3 SDRAM
* - 1 - DDR3 SDRAM device in use
* - 0 - non-DDR3 SDRAM device in use
* Only present in designs that support DDR3.
*/
pub const UDDRC_MSTR_DDR3:u32 = 0x1u32 << 0;

/* (UDDRC_MSTR) Select mDDR SDRAM
* - 1 - Mobile/LPDDR SDRAM device in use
* - 0 - non-mobile SDRAM device in use
* This is only present for designs supporting mDDR.
*/
pub const UDDRC_MSTR_MOBILE:u32 = 0x1u32 << 1;

/* (UDDRC_MSTR) Select LPDDR2 SDRAM
* - 1 - LPDDR2 SDRAM device in use.
* - 0 - non-LPDDR2 device in use
* Present only in designs configured to support LPDDR2.
*/
pub const UDDRC_MSTR_LPDDR2:u32 = 0x1u32 << 2;

/* (UDDRC_MSTR) Select LPDDR3 SDRAM
* - 1 - LPDDR3 SDRAM device in use.
* - 0 - non-LPDDR3 device in use
* Present only in designs configured to support LPDDR3.
*/
pub const UDDRC_MSTR_LPDDR3:u32 = 0x1u32 << 3;

/* (UDDRC_MSTR) Select LPDDR4 SDRAM
* - 1 - LPDDR4 SDRAM device in use.
* - 0 - non-LPDDR4 device in use
* Present only in designs configured to support LPDDR4.
*/
pub const UDDRC_MSTR_LPDDR4:u32 = 0x1u32 << 5;

/* (UDDRC_MSTR) When set, enable interleaved burst mode */
pub const UDDRC_MSTR_BURST_MODE:u32 = 0x1u32 << 8;

/* (UDDRC_MSTR) When set, enable burst-chop (BC4 or 8 on-the-fly) in DDR3/DDR4.
* Burst Chop for Reads is exercised only in HIF configurations
* (UMCTL2_INCL_ARB not set) and if in full bus width mode
* (MSTR.data_bus_width = 00) and if MEMC_BURST_LENGTH=8 or 16.
* Burst Chop for Writes is exercised only if Partial Writes enabled
* (UMCTL2_PARTIAL_WR=1) and if CRC is disabled (CRCPARCTL1.crc_enable = 0).
* If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1),
* burst chop is not supported, and this bit must be set to '0'.
* BC4 (fixed) mode is not supported.
*/
pub const UDDRC_MSTR_BURSTCHOP:u32 = 0x1u32 << 9;

/* (UDDRC_MSTR) If 1, then uMCTL2 uses 2T timing. Otherwise, uses 1T timing.
* In 2T timing, all command signals (except chip select) are held for
* 2 clocks on the SDRAM bus.
* Chip select is asserted on the second cycle of the command
* Note: 2T timing is not supported in LPDDR2/LPDDR3/LPDDR4 mode
* Note: 2T timing is not supported if the configuration parameter
* MEMC_CMD_RTN2IDLE is set
* Note: 2T timing is not supported in DDR4 geardown mode.
* Note: 2T timing is not supported in Shared-AC dual channel mode and
* the register value is don't care.
*/
pub const UDDRC_MSTR_EN_2T_TIMING_MODE:u32 = 0x1u32 << 10;

/* (UDDRC_MSTR) geardown_mode */
pub const UDDRC_MSTR_GEARDOWN_MODE:u32 = 0x1u32 << 11;

/* (UDDRC_MSTR) Selects proportion of DQ bus width that is used by the SDRAM
* - 00 - Full DQ bus width to SDRAM
* - 01 - Half DQ bus width to SDRAM
* - 10 - Quarter DQ bus width to SDRAM
* - 11 - Reserved.
* Note that half bus width mode is only supported when the SDRAM bus width
* is a multiple of 16, and quarter bus width mode is only supported when
* the SDRAM bus width is a multiple of 32 and the configuration parameter
* MEMC_QBUS_SUPPORT is set.
* Bus width refers to DQ bus width (excluding any ECC width).
*/
pub const UDDRC_MSTR_DATA_BUS_WIDTH_POS:u32 = 12;
pub const UDDRC_MSTR_DATA_BUS_WIDTH_MSK:u32 = 0x3u32 << UDDRC_MSTR_DATA_BUS_WIDTH_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MSTR_DATA_BUS_WIDTH (value:u32) -> u32 {
    UDDRC_MSTR_DATA_BUS_WIDTH_MSK & ((value) << UDDRC_MSTR_DATA_BUS_WIDTH_POS)
}

/* (UDDRC_MSTR) Set to 1 when the uMCTL2 and DRAM has to be put in
* DLL-off mode for low frequency operation.
* Set to 0 to put uMCTL2 and DRAM in DLL-on mode for normal frequency operation.
* If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1),
* dll_off_mode is not supported, and this bit must be set to '0'.
*/
pub const UDDRC_MSTR_DLL_OFF_MODE:u32 = 0x1u32 << 15;

/* (UDDRC_MSTR) SDRAM burst length used:
* - 0001 - Burst length of 2 (only supported for mDDR)
* - 0010 - Burst length of 4
* - 0100 - Burst length of 8
* - 1000 - Burst length of 16 (only supported for mDDR, LPDDR2, and LPDDR4)
* All other values are reserved.
* This controls the burst size used to access the SDRAM.
* This must match the burst length mode register setting in the SDRAM.
* (For BC4/8 on-the-fly mode of DDR3 and DDR4, set this field to 0x0100)
* Burst length of 2 is not supported with AXI ports when MEMC_BURST_LENGTH is 8.
* Burst length of 2 is only supported when the controller is operating in 1:1
* frequency mode.
* For DDR3, DDR4 and LPDDR3, this must be set to 0x0100 (BL8).
* For LPDDR4, this must be set to 0x1000 (BL16).
*/
pub const UDDRC_MSTR_BURST_RDWR_BL2:u32 = 1 /* 0001 */;
pub const UDDRC_MSTR_BURST_RDWR_BL4:u32 = 2 /* 0010 */;
pub const UDDRC_MSTR_BURST_RDWR_BL8:u32 = 4 /* 0100 */;
pub const UDDRC_MSTR_BURST_RDWR_BL16:u32 = 8 /* 1000 */;
pub const UDDRC_MSTR_BURST_RDWR_POS:u32 = 16;
pub const UDDRC_MSTR_BURST_RDWR_MSK:u32 = 0xfu32 << UDDRC_MSTR_BURST_RDWR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MSTR_BURST_RDWR(value:u32) -> u32 {
    UDDRC_MSTR_BURST_RDWR_MSK & ((value) << UDDRC_MSTR_BURST_RDWR_POS)
}

/* (UDDRC_MSTR) Choose which registers are used.
* - 0 - Original registers
* - 1 - When UMCTL2_FREQUENCY_NUM=2: FREQ1 registers
*      When UMCTL2_FREQUENCY_NUM>2: Choosen by MSTR2.target_frequency register
*/
pub const UDDRC_MSTR_FREQUENCY_RATIO:u32 = 0x1u32 << 22;

pub const UDDRC_MSTR_ACTIVE_RANKS_0:u32 = 0;
pub const UDDRC_MSTR_ACTIVE_RANKS_1:u32 = 1;
pub const UDDRC_MSTR_ACTIVE_RANKS_2:u32 = 3;
pub const UDDRC_MSTR_ACTIVE_RANKS_4:u32 = 0xf;
pub const UDDRC_MSTR_ACTIVE_RANKS_POS:u32 = 24;
pub const UDDRC_MSTR_ACTIVE_RANKS_MSK:u32 = 0xfu32 << UDDRC_MSTR_ACTIVE_RANKS_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MSTR_ACTIVE_RANKS(value:u32) -> u32 {
    UDDRC_MSTR_ACTIVE_RANKS_MSK & ((value) << UDDRC_MSTR_ACTIVE_RANKS_POS)
}

pub const UDDRC_MSTR_FREQUENCY_MODE:u32 = 0x1u32 << 29;

pub const UDDRC_MSTR_DEVICE_CONFIG_X4:u32 = 0;
pub const UDDRC_MSTR_DEVICE_CONFIG_X8:u32 = 1;
pub const UDDRC_MSTR_DEVICE_CONFIG_X16:u32 = 2;
pub const UDDRC_MSTR_DEVICE_CONFIG_X32:u32 = 3;
pub const UDDRC_MSTR_DEVICE_CONFIG_POS:u32 = 30;
pub const UDDRC_MSTR_DEVICE_CONFIG_MSK:u32 = 0x3u32 << UDDRC_MSTR_DEVICE_CONFIG_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MSTR_DEVICE_CONFIG(value:u32) -> u32 {
    UDDRC_MSTR_DEVICE_CONFIG_MSK & ((value) << UDDRC_MSTR_DEVICE_CONFIG_POS)
}

/* -------- UDDRC_STAT : (UDDRC_REGS Offset: 0x4)
* Operating Mode Status Register --------
*/
/* (UDDRC_STAT) Operating mode.
* This is 3-bits wide in configurations with mDDR/LPDDR2/LPDDR3/LPDDR4/DDR4
* support and 2-bits in all other configurations.
non-mDDR/LPDDR2/LPDDR3/LPDDR4 and non-DDR4 designs:
- 00 - Init
- 01 - Normal
- 10 - Power-down
- 11 - Self refresh
mDDR/LPDDR2/LPDDR3 or DDR4 designs:
- 000 - Init
- 001 - Normal
- 010 - Power-down
- 011 - Self refresh
- 1XX - Deep power-down / Maximum Power Saving Mode
LPDDR4 designs:
- 000 - Init
- 001 - Normal
- 010 - Power-down
- 011 - Self refresh / Self refresh power-down
*/
pub const UDDRC_STAT_OPERATING_MODE_POS:u32 = 0;
pub const UDDRC_STAT_OPERATING_MODE_MSK:u32 = 0x3u32 << UDDRC_STAT_OPERATING_MODE_POS;
pub const UDDRC_STAT_OPERATING_MODE_NORMAL:u32 = 0x1u32;

/* (UDDRC_STAT) Flags if Self Refresh (except LPDDR4) or SR-Powerdown (LPDDR4)
* is entered and if it was under Automatic Self Refresh control only or not.
- 00 - SDRAM is not in Self Refresh (except LPDDR4) or SR-Powerdown (LPDDR4).
If retry is enabled by CRCPARCTRL1.crc_parity_retry_enable, this also indicates
SRE command is still in parity error window or retry is in-progress.
- 11 - SDRAM is in Self Refresh (except LPDDR4) or SR-Powerdown (LPDDR4),
which was caused by Automatic Self Refresh only. If retry is enabled, this
guarantees SRE command is executed correctly without parity error.
- 10 - SDRAM is in Self Refresh (except LPDDR4) or SR-Powerdown (LPDDR4),
which was not caused solely under Automatic Self Refresh control.
It could have been caused by Hardware Low Power Interface and/or Software
(PWRCTL.selfref_sw). If retry is enabled, this guarantees SRE command is
executed correctly without parity error.
- 01 - SDRAM is in Self Refresh, which was caused by PHY Master Request.
*/
pub const UDDRC_STAT_SELFREF_TYPE_POS:u32 = 4;
pub const UDDRC_STAT_SELFREF_TYPE_MSK:u32 = 0x3u32 << UDDRC_STAT_SELFREF_TYPE_POS;

/* (UDDRC_STAT) Self refresh with CAMs not empty. Set to 1 when Self Refresh
* is entered but CAMs are not drained. Cleared after exiting Self Refresh.
*/
pub const UDDRC_STAT_SELFREF_CAM_NOT_EMPTY:u32 = 0x1u32 << 12;

/* -------- UDDRC_MRCTRL0 : (UDDRC_REGS Offset: 0x10)
* Mode Register Read/Write Control Register 0. --------
*/
/* (UDDRC_MRCTRL0) Indicates whether the mode register operation is
*read or write. Only used for LPDDR2/LPDDR3/LPDDR4/DDR4.
- 0 - Write
- 1 - Read
*/
pub const UDDRC_MRCTRL0_MR_TYPE:u32 = 0x1u32 << 0;

/* (UDDRC_MRCTRL0) Controls which rank is accessed by MRCTRL0.mr_wr.
* Normally, it is desired to access all ranks, so all bits should be set to 1.
* However, for multi-rank UDIMMs/RDIMMs/LRDIMMs which implement address
* mirroring, it may be necessary to access ranks individually.
* Examples (assume uMCTL2 is configured for 4 ranks):
- 0x1 - select rank 0 only
- 0x2 - select rank 1 only
- 0x5 - select ranks 0 and 2
- 0xA - select ranks 1 and 3
- 0xF - select ranks 0, 1, 2 and 3
*/
pub const UDDRC_MRCTRL0_MR_RANK_POS:u32 = 4;
pub const UDDRC_MRCTRL0_MR_RANK_MSK:u32 = 0xfu32 << UDDRC_MRCTRL0_MR_RANK_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MRCTRL0_MR_RANK (value:u32) -> u32 {
    UDDRC_MRCTRL0_MR_RANK_MSK & ((value) << UDDRC_MRCTRL0_MR_RANK_POS)
}

/* (UDDRC_MRCTRL0) Address of the mode register that is to be written to.
- 0000 - MR0
- 0001 - MR1
- 0010 - MR2
- 0011 - MR3
- 0100 - MR4
- 0101 - MR5
- 0110 - MR6
- 0111 - MR7
* Don't Care for LPDDR2/LPDDR3/LPDDR4 (see MRCTRL1.mr_data for mode register
* addressing in LPDDR2/LPDDR3/LPDDR4)
* This signal is also used for writing to control words of the register chip
* on RDIMMs/LRDIMMs. In that case, it corresponds to the bank address bits
* sent to the RDIMM/LRDIMM
* In case of DDR4, the bit[3:2] corresponds to the bank group bits.
* Therefore, the bit[3] as well as the bit[2:0] must be set to an appropriate
* value which is considered both the Address Mirroring of UDIMMs/RDIMMs/LRDIMMs
* and the Output Inversion of RDIMMs/LRDIMMs.
*/
pub const UDDRC_MRCTRL0_MR_ADDR_POS:u32 = 12;
pub const UDDRC_MRCTRL0_MR_ADDR_MSK:u32 = 0xfu32 << UDDRC_MRCTRL0_MR_ADDR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MRCTRL0_MR_ADDR (value:u32) -> u32 {
    UDDRC_MRCTRL0_MR_ADDR_MSK & ((value) << UDDRC_MRCTRL0_MR_ADDR_POS)
}

/* (UDDRC_MRCTRL0) Setting this register bit to 1 triggers a mode register
* read or write operation. When the MR operation is complete, the uMCTL2
* automatically clears this bit. The other register fields of this register
* must be written in a separate APB transaction, before setting this mr_wr bit.
* It is recommended NOT to set this signal if in Init, Deep power-down
* or MPSM operating modes.
*/
pub const UDDRC_MRCTRL0_MR_WR:u32 = 0x1u32 << 31;

/* -------- UDDRC_MRCTRL1 : (UDDRC_REGS Offset: 0x14)
* Mode Register Read/Write Control Register 1 --------
*/
/* (UDDRC_MRCTRL1) Mode register write data for all
* non-LPDDR2/non-LPDDR3/non-LPDDR4 modes.
* For LPDDR2/LPDDR3/LPDDR4, MRCTRL1[15:0] are interpreted as [15:8] MR Address
* [7:0] MR data for writes, don't care for reads.
* This is 18-bits wide in configurations with DDR4 support and
* 16-bits in all other configurations.
*/
pub const UDDRC_MRCTRL1_MR_DATA_POS:u32 = 0;
pub const UDDRC_MRCTRL1_MR_DATA_MSK:u32 = 0x3ffffu32 << UDDRC_MRCTRL1_MR_DATA_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_MRCTRL1_MR_DATA (value:u32) -> u32 {
    UDDRC_MRCTRL1_MR_DATA_MSK & ((value) << UDDRC_MRCTRL1_MR_DATA_POS)
}

/* -------- UDDRC_MRSTAT : (UDDRC_REGS Offset: 0x18)
* Mode Register Read/Write Status Register --------
*/
/* (UDDRC_MRSTAT) The SoC core may initiate a MR write operation only if
* this signal is low. This signal goes high in the clock after the uMCTL2
* accepts the MRW/MRR request. It goes low when the MRW/MRR command is issued
* to the SDRAM. It is recommended not to perform MRW/MRR commands when
* 'MRSTAT.mr_wr_busy' is high.
- 0 - Indicates that the SoC core can initiate a mode register write operation
- 1 - Indicates that mode register write operation is in progress
*/

/* -------- UDDRC_DERATEEN : (UDDRC_REGS Offset: 0x20)
* Temperature Derate Enable Register --------
*/
pub const UDDRC_MRSTAT_MR_WR_BUSY:u32 = 0x1u32 << 0;

/* (UDDRC_DERATEEN) Enables derating
- 0 - Timing parameter derating is disabled
- 1 - Timing parameter derating is enabled using MR4 read value.
* Present only in designs configured to support LPDDR2/LPDDR3/LPDDR4
* This field must be set to '0' for non-LPDDR2/LPDDR3/LPDDR4 mode.
*/
pub const UDDRC_DERATEEN_DERATE_ENABLE:u32 = 0x1u32 << 0;

/* (UDDRC_DERATEEN) Derate value
- 0 - Derating uses +1.
- 1 - Derating uses +2.
* Present only in designs configured to support LPDDR2/LPDDR3/LPDDR4
* Set to 0 for all LPDDR2 speed grades as derating value of +1.875 ns is less
* than a core_ddrc_core_clk period.
* For LPDDR3/4, if the period of core_ddrc_core_clk is less than 1.875ns,
* this register field should be set to 1; otherwise it should be set to 0.
*/
pub const UDDRC_DERATEEN_DERATE_VALUE_POS:u32 = 1;
pub const UDDRC_DERATEEN_DERATE_VALUE_MSK:u32 = 0x3u32 << UDDRC_DERATEEN_DERATE_VALUE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DERATEEN_DERATE_VALUE (value:u32) -> u32 {
    UDDRC_DERATEEN_DERATE_VALUE_MSK & ((value) << UDDRC_DERATEEN_DERATE_VALUE_POS)
}

/* (UDDRC_DERATEEN) Derate byte
* Present only in designs configured to support LPDDR2/LPDDR3/LPDDR4
* Indicates which byte of the MRR data is used for derating.
* The maximum valid value depends on MEMC_DRAM_TOTAL_DATA_WIDTH.
*/
pub const UDDRC_DERATEEN_DERATE_BYTE_POS:u32 = 4;
pub const UDDRC_DERATEEN_DERATE_BYTE_MSK:u32 = 0xfu32 << UDDRC_DERATEEN_DERATE_BYTE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DERATEEN_DERATE_BYTE (value:u32) -> u32 {
    UDDRC_DERATEEN_DERATE_BYTE_MSK & ((value) << UDDRC_DERATEEN_DERATE_BYTE_POS)
}

/* -------- UDDRC_DERATEINT : (UDDRC_REGS Offset: 0x24)
* Temperature Derate Interval Register --------
*/
/* (UDDRC_DERATEINT) Interval between two MR4 reads, used to derate
* the timing parameters.
* Present only in designs configured to support LPDDR2/LPDDR3/LPDDR4.
* This register must not be set to zero.
* Unit: DFI clock cycle.
*/
pub const UDDRC_DERATEINT_MR4_READ_INTERVAL_POS:u32 = 0;
pub const UDDRC_DERATEINT_MR4_READ_INTERVAL_MSK:u32 = 0xffffffffu32 << UDDRC_DERATEINT_MR4_READ_INTERVAL_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DERATEINT_MR4_READ_INTERVAL (value:u32) -> u32 {
    UDDRC_DERATEINT_MR4_READ_INTERVAL_MSK & ((value) << UDDRC_DERATEINT_MR4_READ_INTERVAL_POS)
}

/* -------- UDDRC_PWRCTL : (UDDRC_REGS Offset: 0x30)
* Low Power Control Register --------
*/
/* (UDDRC_PWRCTL) If true then the uMCTL2 puts the SDRAM into Self Refresh
* after a programmable number of cycles
* "maximum idle clocks before Self Refresh (PWRTMG.selfref_to_x32)".
* This register bit may be re-programmed during the course of normal operation.
*/
pub const UDDRC_PWRCTL_SELFREF_EN:u32 = 0x1u32 << 0;

/* (UDDRC_PWRCTL) If true then the uMCTL2 goes into power-down after
* a programmable number of cycles "maximum idle clocks before power down"
* (PWRTMG.powerdown_to_x32).
* This register bit may be re-programmed during the course of normal operation.
*/
pub const UDDRC_PWRCTL_POWERDOWN_EN:u32 = 0x1u32 << 1;

/* (UDDRC_PWRCTL) When this is 1, uMCTL2 puts the SDRAM into deep power-down
* mode when the transaction store is empty.
* This register must be reset to '0' to bring uMCTL2 out of deep power-down
* mode. Controller performs automatic SDRAM initialization on deep power-down
* exit.
* Present only in designs configured to support mDDR or LPDDR2 or LPDDR3.
* For non-mDDR/non-LPDDR2/non-LPDDR3, this register should not be set to 1.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PWRCTL_DEEPPOWERDOWN_EN:u32 = 0x1u32 << 2;

/* (UDDRC_PWRCTL) Enable the assertion of dfi_dram_clk_disable whenever
* a clock is not required by the SDRAM.
* If set to 0, dfi_dram_clk_disable is never asserted.
* Assertion of dfi_dram_clk_disable is as follows:
* In DDR2/DDR3, can only be asserted in Self Refresh.
* In DDR4, can be asserted in following:
- in Self Refresh.
- in Maximum Power Saving Mode
* In mDDR/LPDDR2/LPDDR3, can be asserted in following:
- in Self Refresh
- in Power Down
- in Deep Power Down
- during Normal operation (Clock Stop)
* In LPDDR4, can be asserted in following:
- in Self Refresh Power Down
- in Power Down
- during Normal operation (Clock Stop)
*/
pub const UDDRC_PWRCTL_EN_DFI_DRAM_CLK_DISABLE:u32 = 0x1u32 << 3;

/* (UDDRC_PWRCTL) A value of 1 to this register causes system to move to
* Self Refresh state immediately, as long as it is not in INIT or DPD/MPSM
* operating_mode. This is referred to as Software Entry/Exit to Self Refresh.
- 1 - Software Entry to Self Refresh
- 0 - Software Exit from Self Refresh
*/
pub const UDDRC_PWRCTL_SELFREF_SW:u32 = 0x1u32 << 5;

/* (UDDRC_PWRCTL) Indicates whether skipping CAM draining is allowed when
* entering Self-Refresh.
* This register field cannot be modified while PWRCTL.selfref_sw==1.
- 0 - CAMs must be empty before entering SR
- 1 - CAMs are not emptied before entering SR
*/
pub const UDDRC_PWRCTL_DIS_CAM_DRAIN_SELFREF:u32 = 0x1u32 << 7;

/* -------- UDDRC_PWRTMG : (UDDRC_REGS Offset: 0x34)
* Low Power Timing Register --------
*/
/* (UDDRC_PWRTMG) After this many clocks of the DDRC command channel being idle
* the uMCTL2 automatically puts the SDRAM into power-down.
* The DDRC command channel is considered idle when there are no HIF commands
* outstanding. This must be enabled in the PWRCTL.powerdown_en.
* Unit: Multiples of 32 DFI clocks
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PWRTMG_POWERDOWN_TO_X32_POS:u32 = 0;
pub const UDDRC_PWRTMG_POWERDOWN_TO_X32_MSK:u32 = 0x1fu32 << UDDRC_PWRTMG_POWERDOWN_TO_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PWRTMG_POWERDOWN_TO_X32 (value:u32) -> u32 {
    UDDRC_PWRTMG_POWERDOWN_TO_X32_MSK & ((value) << UDDRC_PWRTMG_POWERDOWN_TO_X32_POS)
}

/* (UDDRC_PWRTMG) Minimum deep power-down time.
* For mDDR, value from the JEDEC specification is 0 as mDDR exits from
* deep power-down mode immediately after PWRCTL.deeppowerdown_en is de-asserted.
* For LPDDR2/LPDDR3, value from the JEDEC specification is 500us.
* Unit: Multiples of 4096 DFI clocks.
* Present only in designs configured to support mDDR, LPDDR2 or LPDDR3.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PWRTMG_T_DPD_X4096_POS:u32 = 8;
pub const UDDRC_PWRTMG_T_DPD_X4096_MSK:u32 = 0xffu32 << UDDRC_PWRTMG_T_DPD_X4096_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PWRTMG_T_DPD_X4096 (value:u32) -> u32 {
    UDDRC_PWRTMG_T_DPD_X4096_MSK & ((value) << UDDRC_PWRTMG_T_DPD_X4096_POS)
}

/* (UDDRC_PWRTMG) After this many clocks of the DDRC command channel being idle
* the uMCTL2 automatically puts the SDRAM into Self Refresh.
* The DDRC command channel is considered idle when there are no HIF
* commands outstanding. This must be enabled in the PWRCTL.selfref_en.
* Unit: Multiples of 32 DFI clocks.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PWRTMG_SELFREF_TO_X32_POS:u32 = 16;
pub const UDDRC_PWRTMG_SELFREF_TO_X32_MSK:u32 = 0xffu32 << UDDRC_PWRTMG_SELFREF_TO_X32_POS;

#[allow(non_snake_case)]
pub const fn UDDRC_PWRTMG_SELFREF_TO_X32 (value:u32) -> u32 {
    UDDRC_PWRTMG_SELFREF_TO_X32_MSK & ((value) << UDDRC_PWRTMG_SELFREF_TO_X32_POS)
}

/* -------- UDDRC_HWLPCTL : (UDDRC_REGS Offset: 0x38)
* Hardware Low Power Control Register --------
*/
/* (UDDRC_HWLPCTL) Enable for Hardware Low Power Interface. */
pub const UDDRC_HWLPCTL_HW_LP_EN:u32 = 0x1u32 << 0;

/* (UDDRC_HWLPCTL) When this bit is programmed to 1 the cactive_in_ddrc pin
* of the DDRC can be used to exit from the automatic clock stop,
* automatic power down or automatic self-refresh modes.
* Note, it will not cause exit of Self-Refresh that was caused by
* Hardware Low Power Interface and/or Software (PWRCTL.selfref_sw).
*/
pub const UDDRC_HWLPCTL_HW_LP_EXIT_IDLE_EN:u32 = 0x1u32 << 1;

/* (UDDRC_HWLPCTL) Hardware idle period. The cactive_ddrc output is driven low
* if the DDRC command channel is idle for hw_lp_idle * 32 cycles
* if not in INIT or DPD/MPSM operating_mode.
* The DDRC command channel is considered idle when there are no HIF
* commands outstanding.
* The hardware idle function is disabled when hw_lp_idle_x32=0.
* Unit: Multiples of 32 DFI clocks.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_HWLPCTL_HW_LP_IDLE_X32_POS:u32 = 16;
pub const UDDRC_HWLPCTL_HW_LP_IDLE_X32_MSK:u32 = 0xfffu32 << UDDRC_HWLPCTL_HW_LP_IDLE_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_HWLPCTL_HW_LP_IDLE_X32 (value:u32) -> u32 {
    UDDRC_HWLPCTL_HW_LP_IDLE_X32_MSK & ((value) << UDDRC_HWLPCTL_HW_LP_IDLE_X32_POS)
}

/* -------- UDDRC_RFSHCTL0 : (UDDRC_REGS Offset: 0x50)
* Refresh Control Register 0 --------
*/
/* (UDDRC_RFSHCTL0)
- 1 - Per bank refresh;
- 0 - All bank refresh.
* Per bank refresh allows traffic to flow to other banks.
* Per bank refresh is not supported by all LPDDR2 devices but should be
* supported by all LPDDR3/LPDDR4 devices.
* Present only in designs configured to support LPDDR2/LPDDR3/LPDDR4
*/
pub const UDDRC_RFSHCTL0_PER_BANK_REFRESH:u32 = 0x1u32 << 2;

/* (UDDRC_RFSHCTL0) The programmed value + 1 is the number of refresh timeouts
* that is allowed to accumulate before traffic is blocked and the refreshes
* are forced to execute. Closing pages to perform a refresh is a one-time
* penalty that must be paid for each group of refreshes.
* Therefore, performing refreshes in a burst reduces the per-refresh penalty
* of these page closings. Higher numbers for RFSHCTL.refresh_burst slightly
* increases utilization; lower numbers decreases the worst-case
* latency associated with refreshes.
- 0 - single refresh
- 1 - burst-of-2 refresh
- 7 - burst-of-8 refresh
* For information on burst refresh feature refer to section 3.9
* of DDR2 JEDEC specification - JESD79-2F.pdf.
* For DDR2/3, the refresh is always per-rank and not per-bank. The rank refresh
* can be accumulated over 8*tREFI cycles using the burst refresh feature.
* In DDR4 mode, according to Fine Granularity feature, 8 refreshes can be
* postponed in 1X mode, 16 refreshes in 2X mode and 32 refreshes in 4X mode.
* If using PHY-initiated updates, care must be taken in the setting of
* RFSHCTL0.refresh_burst, to ensure that tRFCmax is not violated due to a
* PHY-initiated update occurring shortly before a refresh burst was due.
* In this situation, the refresh burst will be delayed until the PHY-initiated
* update is complete.
*/

pub const UDDRC_RFSHCTL0_REFRESH_BURST_POS:u32 = 4;
pub const UDDRC_RFSHCTL0_REFRESH_BURST_MSK:u32 = 0x1fu32 << UDDRC_RFSHCTL0_REFRESH_BURST_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHCTL0_REFRESH_BURST (value:u32) -> u32 {
    UDDRC_RFSHCTL0_REFRESH_BURST_MSK & ((value) << UDDRC_RFSHCTL0_REFRESH_BURST_POS)
}

/* (UDDRC_RFSHCTL0) If the refresh timer (tRFCnom, also known as tREFI)
* has expired at least once, but it has not expired (RFSHCTL0.refresh_burst+1)
* times yet, then a speculative refresh may be performed.
* A speculative refresh is a refresh performed at a time when refresh would be
* useful, but before it is absolutely required. When the SDRAM bus is idle for
* a period of time determined by this RFSHCTL0.refresh_to_x32 and the refresh
* timer has expired at least once since the last refresh, then a speculative
* refresh is performed. Speculative refreshes continues successively until
* there are no refreshes pending or until new reads or writes are issued
* to the uMCTL2.
* FOR PERFORMANCE ONLY.
* Unit: Multiples of 32 DFI clocks.
*/
pub const UDDRC_RFSHCTL0_REFRESH_TO_X32_POS:u32 = 12;
pub const UDDRC_RFSHCTL0_REFRESH_TO_X32_MSK:u32 = 0x1fu32 << UDDRC_RFSHCTL0_REFRESH_TO_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHCTL0_REFRESH_TO_X32 (value:u32) -> u32 {
    UDDRC_RFSHCTL0_REFRESH_TO_X32_MSK & ((value) << UDDRC_RFSHCTL0_REFRESH_TO_X32_POS)
}

/* (UDDRC_RFSHCTL0) Threshold value in number of DFI clock cycles before the
* critical refresh or page timer expires. A critical refresh is to be issued
* before this threshold is reached. It is recommended that this not be changed
* from the default value, currently shown as 0x2. It must always be less than
* internally used t_rfc_nom_x32. Note that, in LPDDR2/LPDDR3/LPDDR4,
* internally used t_rfc_nom_x32 may be equal to RFSHTMG.t_rfc_nom_x32>>2
* if derating is enabled (DERATEEN.derate_enable=1).
* Otherwise, internally used t_rfc_nom_x32 will be equal to RFSHTMG.t_rfc_nom_x32.
* Unit: Multiples of 32 DFI clocks.
*/
pub const UDDRC_RFSHCTL0_REFRESH_MARGIN_POS:u32 = 20;
pub const UDDRC_RFSHCTL0_REFRESH_MARGIN_MSK:u32 = 0xfu32 << UDDRC_RFSHCTL0_REFRESH_MARGIN_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHCTL0_REFRESH_MARGIN (value:u32) -> u32 {
    UDDRC_RFSHCTL0_REFRESH_MARGIN_MSK & ((value) << UDDRC_RFSHCTL0_REFRESH_MARGIN_POS)
}

/* -------- UDDRC_RFSHCTL3 : (UDDRC_REGS Offset: 0x60)
* Refresh Control Register 3 --------
*/
/* (UDDRC_RFSHCTL3) When '1', disable auto-refresh generated by the uMCTL2.
* When auto-refresh is disabled, the SoC core must generate refreshes using
* the registers DBGCMD.rankn_refresh.
* When dis_auto_refresh transitions from 0 to 1, any pending refreshes
* are immediately scheduled by the uMCTL2.
* If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1),
* disable auto-refresh is not supported, and this bit must be set to '0'.
* (DDR4 only) If FGR mode is enabled (RFSHCTL3.refresh_mode > 0),
* disable auto-refresh is not supported, and this bit must be set to '0'.
* This register field is changeable on the fly.
*/
pub const UDDRC_RFSHCTL3_DIS_AUTO_REFRESH:u32 = 0x1u32 << 0;

/* (UDDRC_RFSHCTL3) Toggle this signal (either from 0 to 1 or from 1 to 0)
* to indicate that the refresh register(s) have been updated.
* refresh_update_level must not be toggled when the DDRC is in reset
* (core_ddrc_rstn = 0).
* The refresh register(s) are automatically updated when exiting reset.
*/
pub const UDDRC_RFSHCTL3_REFRESH_UPDATE_LEVEL:u32 = 0x1u32 << 1;

/* -------- UDDRC_RFSHTMG : (UDDRC_REGS Offset: 0x64)
* Refresh Timing Register --------
*/
/* (UDDRC_RFSHTMG) tRFC (min): Minimum time from refresh to refresh or activate.
* When the controller is operating in 1:1 mode, t_rfc_min should be set to
* RoundUp(tRFCmin/tCK).
* When the controller is operating in 1:2 mode, t_rfc_min should be set to
* RoundUp(RoundUp(tRFCmin/tCK)/2).
* In LPDDR2/LPDDR3/LPDDR4 mode:
- if using all-bank refreshes, the tRFCmin value in the above equations is
equal to tRFCab
- if using per-bank refreshes, the tRFCmin value in the above equations is
equal to tRFCpb
* In DDR4 mode, the tRFCmin value in the above equations is different depending
* on the refresh mode (fixed 1X,2X,4X) and the device density.
* The user should program the appropriate value from the spec based on the
* 'refresh_mode' and the device density that is used.
* Unit: Clocks.
*/
pub const UDDRC_RFSHTMG_T_RFC_MIN_POS:u32 = 0;
pub const UDDRC_RFSHTMG_T_RFC_MIN_MSK:u32 = 0x3ffu32 << UDDRC_RFSHTMG_T_RFC_MIN_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHTMG_T_RFC_MIN (value:u32) -> u32 {
    UDDRC_RFSHTMG_T_RFC_MIN_MSK & ((value) << UDDRC_RFSHTMG_T_RFC_MIN_POS)
}

/* (UDDRC_RFSHTMG) Used only when LPDDR3 memory type is connected.
* Should only be changed when uMCTL2 is in reset. Specifies whether to use the
* tREFBW parameter (required by some LPDDR3 devices which comply with earlier
* versions of the LPDDR3 JEDEC specification) or not:
- 0 - tREFBW parameter not used
- 1 - tREFBW parameter used
*/
pub const UDDRC_RFSHTMG_LPDDR3_TREFBW_EN:u32 = 0x1u32 << 15;

/* (UDDRC_RFSHTMG) tREFI: Average time interval between refreshes per rank
* (Specification: 7.8us for DDR2, DDR3 and DDR4. See JEDEC specification for
* mDDR, LPDDR2, LPDDR3 and LPDDR4).
* For LPDDR2/LPDDR3/LPDDR4:
- if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0),
this register should be set to tREFIab
- if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1),
this register should be set to tREFIpb
* When the controller is operating in 1:2 frequency ratio mode,
* program this to (tREFI/2), no rounding up.
*
* In DDR4 mode, tREFI value is different depending on the refresh mode.
* The user should program the appropriate value from the spec based on the
* value programmed in the refresh mode register.
* Note that RFSHTMG.t_rfc_nom_x32 * 32 must be greater than RFSHTMG.t_rfc_min,
* and RFSHTMG.t_rfc_nom_x32 must be greater than 0x1.
- Non-DDR4 or DDR4 Fixed 1x mode: RFSHTMG.t_rfc_nom_x32 must be less than
or equal to 0xFFE.
- DDR4 Fixed 2x mode: RFSHTMG.t_rfc_nom_x32 must be less than or equal to 0x7FF.
- DDR4 Fixed 4x mode: RFSHTMG.t_rfc_nom_x32 must be less than or equal to 0x3FF.
* Unit: Multiples of 32 clocks.
*/
pub const UDDRC_RFSHTMG_T_RFC_NOM_X32_POS:u32 = 16;
pub const UDDRC_RFSHTMG_T_RFC_NOM_X32_MSK:u32 = 0xfffu32 << UDDRC_RFSHTMG_T_RFC_NOM_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHTMG_T_RFC_NOM_X32 (value:u32) -> u32 {
    UDDRC_RFSHTMG_T_RFC_NOM_X32_MSK & ((value) << UDDRC_RFSHTMG_T_RFC_NOM_X32_POS)
}

pub const UDDRC_RFSHTMG1_T_RFC_MIN_DLR_POS:u32 = 0;
pub const UDDRC_RFSHTMG1_T_RFC_MIN_DLR_MSK:u32 = 0xffu32 << UDDRC_RFSHTMG1_T_RFC_MIN_DLR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHTMG1_T_RFC_MIN_DLR_MIN (value:u32) -> u32 {
    UDDRC_RFSHTMG1_T_RFC_MIN_DLR_MSK & ((value) << UDDRC_RFSHTMG1_T_RFC_MIN_DLR_POS)
}

pub const UDDRC_RFSHTMG1_T_PBR2PBR_POS:u32 = 16;
pub const UDDRC_RFSHTMG1_T_PBR2PBR_MSK:u32 = 0xffu32 << UDDRC_RFSHTMG1_T_PBR2PBR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_RFSHTMG1_T_PBR2PBR (value:u32) -> u32 {
    UDDRC_RFSHTMG1_T_PBR2PBR_MSK & ((value) << UDDRC_RFSHTMG1_T_PBR2PBR_POS)
}

/* -------- UDDRC_CRCPARCTL0 : (UDDRC_REGS Offset: 0xC0)
* CRC Parity Control Register0 --------
*/
/* (UDDRC_CRCPARCTL0) Interrupt enable bit for DFI alert error.
* If this bit is set, any parity/CRC error detected on the dfi_alert_n input
* will result in an interrupt being set on CRCPARSTAT.dfi_alert_err_int.
*/
pub const UDDRC_CRCPARCTL0_DFI_ALERT_ERR_INT_EN:u32 = 0x1u32 << 0;

/* (UDDRC_CRCPARCTL0) Interrupt clear bit for DFI alert error.
* If this bit is set, the alert error interrupt on CRCPARSTAT.dfi_alert_err_int
* will be cleared. When the clear operation is complete,
* the uMCTL2 automatically clears this bit.
*/
pub const UDDRC_CRCPARCTL0_DFI_ALERT_ERR_INT_CLR:u32 = 0x1u32 << 1;

/* (UDDRC_CRCPARCTL0) DFI alert error count clear.
* Clear bit for DFI alert error counter. Asserting this bit will clear the DFI
* alert error counter, CRCPARSTAT.dfi_alert_err_cnt.
* When the clear operation is complete,
* the uMCTL2 automatically clears this bit.
*/
pub const UDDRC_CRCPARCTL0_DFI_ALERT_ERR_CNT_CLR:u32 = 0x1u32 << 2;

/* -------- UDDRC_CRCPARSTAT : (UDDRC_REGS Offset: 0xCC)
* CRC Parity Status Register --------
*/
/* (UDDRC_CRCPARSTAT) DFI alert error count.
* If a parity/CRC error is detected on dfi_alert_n, this counter be incremented.
* This is independent of the setting of CRCPARCTL0.dfi_alert_err_int_en.
* It will saturate at 0xFFFF, and can be cleared by asserting
* CRCPARCTL0.dfi_alert_err_cnt_clr.
*/
pub const UDDRC_CRCPARSTAT_DFI_ALERT_ERR_CNT_POS:u32 = 0;
pub const UDDRC_CRCPARSTAT_DFI_ALERT_ERR_CNT_MSK:u32 = 0xffffu32 << UDDRC_CRCPARSTAT_DFI_ALERT_ERR_CNT_POS;

/* (UDDRC_CRCPARSTAT) DFI alert error interrupt.
* If a parity/CRC error is detected on dfi_alert_n, and the interrupt
* is enabled by CRCPARCTL0.dfi_alert_err_int_en, this interrupt bit will be set.
* It will remain set until cleared by CRCPARCTL0.dfi_alert_err_int_clr
*/
pub const UDDRC_CRCPARSTAT_DFI_ALERT_ERR_INT:u32 = 0x1u32 << 16;

/* -------- UDDRC_INIT0 : (UDDRC_REGS Offset: 0xD0)
* SDRAM Initialization Register 0 --------
*/
/* (UDDRC_INIT0) Cycles to wait after reset before driving CKE high to start
* the SDRAM initialization sequence.
* Unit: 1024 DFI clock cycles.
* DDR2 specifications typically require this to be programmed for
* a delay of >= 200 us.
* LPDDR2/LPDDR3: tINIT1 of 100 ns (min)
* LPDDR4: tINIT3 of 2 ms (min)
* When the controller is operating in 1:2 frequency ratio mode, program this
* to JEDEC spec value divided by 2, and round it up to the next integer value.
* For DDR3/DDR4 RDIMMs, this should include the time needed to satisfy tSTAB
*/
pub const UDDRC_INIT0_PRE_CKE_X1024_POS:u32 = 0;
pub const UDDRC_INIT0_PRE_CKE_X1024_MSK:u32 = 0xfffu32 << UDDRC_INIT0_PRE_CKE_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT0_PRE_CKE_X1024 (value:u32) -> u32 {
    UDDRC_INIT0_PRE_CKE_X1024_MSK & ((value) << UDDRC_INIT0_PRE_CKE_X1024_POS)
}

/* (UDDRC_INIT0) Cycles to wait after driving CKE high to start
* the SDRAM initialization sequence.
* Unit: 1024 DFI clock cycles.
* DDR2 typically requires a 400 ns delay, requiring this value to be
* programmed to 2 at all clock speeds.
* LPDDR2/LPDDR3 typically requires this to be programmed for a delay of 200 us.
* LPDDR4 typically requires this to be programmed for a delay of 2 us.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* JEDEC spec value divided by 2, and round it up to the next integer value.
*/
pub const UDDRC_INIT0_POST_CKE_X1024_POS:u32 = 16;
pub const UDDRC_INIT0_POST_CKE_X1024_MSK:u32 = 0x3ffu32 << UDDRC_INIT0_POST_CKE_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT0_POST_CKE_X1024 (value:u32) -> u32 {
    UDDRC_INIT0_POST_CKE_X1024_MSK & ((value) << UDDRC_INIT0_POST_CKE_X1024_POS)
}

/* (UDDRC_INIT0) If lower bit is enabled the SDRAM initialization routine
* is skipped. The upper bit decides what state the controller starts up
* in when reset is removed
- 00 - SDRAM Intialization routine is run after power-up
- 01 - SDRAM Intialization routine is skipped after power-up.
Controller starts up in Normal Mode
- 11 - SDRAM Intialization routine is skipped after power-up.
Controller starts up in Self-refresh Mode
- 10 - SDRAM Intialization routine is run after power-up.
*/
pub const UDDRC_INIT0_SKIP_DRAM_INIT_POS:u32 = 30;
pub const UDDRC_INIT0_SKIP_DRAM_INIT_MSK:u32 = 0x3u32 << UDDRC_INIT0_SKIP_DRAM_INIT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT0_SKIP_DRAM_INIT (value:u32) -> u32 {
    UDDRC_INIT0_SKIP_DRAM_INIT_MSK & ((value) << UDDRC_INIT0_SKIP_DRAM_INIT_POS)
}

/* -------- UDDRC_INIT1 : (UDDRC_REGS Offset: 0xD4)
* SDRAM Initialization Register 1 --------
*/
/* (UDDRC_INIT1) Wait period before driving the OCD complete command to SDRAM.
* Unit: Counts of a global timer that pulses every 32 DFI clock cycles.
* There is no known specific requirement for this; it may be set to zero.
*/
pub const UDDRC_INIT1_PRE_OCD_X32_POS:u32 = 0;
pub const UDDRC_INIT1_PRE_OCD_X32_MSK:u32 = 0xfu32 << UDDRC_INIT1_PRE_OCD_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT1_PRE_OCD_X32 (value:u32) -> u32 {
    UDDRC_INIT1_PRE_OCD_X32_MSK & ((value) << UDDRC_INIT1_PRE_OCD_X32_POS)
}

/* (UDDRC_INIT1) Number of cycles to assert SDRAM reset signal during
* init sequence.
* This is only present for designs supporting DDR3, DDR4 or LPDDR4 devices.
* For use with a Synopsys DDR PHY, this should be set to a minimum of 1.
* When the controller is operating in 1:2 frequency ratio mode, program this
* to JEDEC spec value divided by 2, and round it up to the next integer value.
* Unit: 1024 DFI clock cycles.
*/
pub const UDDRC_INIT1_DRAM_RSTN_X1024_POS:u32 = 16;
pub const UDDRC_INIT1_DRAM_RSTN_X1024_MSK:u32 = 0x1ffu32 << UDDRC_INIT1_DRAM_RSTN_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT1_DRAM_RSTN_X1024 (value:u32) -> u32 {
    UDDRC_INIT1_DRAM_RSTN_X1024_MSK & ((value) << UDDRC_INIT1_DRAM_RSTN_X1024_POS)
}

/* -------- UDDRC_INIT2 : (UDDRC_REGS Offset: 0xD8)
* SDRAM Initialization Register 2 --------
*/
/* (UDDRC_INIT2) Time to wait after the first CKE high, tINIT2.
* Present only in designs configured to support LPDDR2/LPDDR3.
* LPDDR2/LPDDR3 typically requires 5 x tCK delay.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to JEDEC spec value divided by 2, and round it up to the
* next integer value.
* Unit: DFI clock cycles.
*/
pub const UDDRC_INIT2_MIN_STABLE_CLOCK_X1_POS:u32 = 0;
pub const UDDRC_INIT2_MIN_STABLE_CLOCK_X1_MSK:u32 = 0xfu32 << UDDRC_INIT2_MIN_STABLE_CLOCK_X1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT2_MIN_STABLE_CLOCK_X1 (value:u32) -> u32 {
    UDDRC_INIT2_MIN_STABLE_CLOCK_X1_MSK & ((value) << UDDRC_INIT2_MIN_STABLE_CLOCK_X1_POS)
}

/* (UDDRC_INIT2) Idle time after the reset command, tINIT4.
* Present only in designs configured to support LPDDR2.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to JEDEC spec value divided by 2, and round it up to
* the next integer value.
* Unit: 32 DFI clock cycles.
*/
pub const UDDRC_INIT2_IDLE_AFTER_RESET_X32_POS:u32 = 8;
pub const UDDRC_INIT2_IDLE_AFTER_RESET_X32_MSK:u32 = 0xffu32 << UDDRC_INIT2_IDLE_AFTER_RESET_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT2_IDLE_AFTER_RESET_X32 (value:u32) -> u32 {
    UDDRC_INIT2_IDLE_AFTER_RESET_X32_MSK & ((value) << UDDRC_INIT2_IDLE_AFTER_RESET_X32_POS)
}

/* -------- UDDRC_INIT3 : (UDDRC_REGS Offset: 0xDC)
* SDRAM Initialization Register 3 --------
*/
/* (UDDRC_INIT3) DDR2: Value to write to EMR register.
* Bits 9:7 are for OCD and the setting in this register is ignored.
* The uMCTL2 sets those bits appropriately.
* DDR3/DDR4: Value to write to MR1 register  Set bit 7 to 0.
* If PHY-evaluation mode training is enabled, this bit is set appropriately by
* the uMCTL2 during write leveling.
* mDDR: Value to write to EMR register.
* LPDDR2/LPDDR3/LPDDR4 - Value to write to MR2 register
*/
pub const UDDRC_INIT3_EMR_POS:u32 = 0;
pub const UDDRC_INIT3_EMR_MSK:u32 = 0xffffu32 << UDDRC_INIT3_EMR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT3_EMR(value:u32) -> u32 {
    UDDRC_INIT3_EMR_MSK & ((value) << UDDRC_INIT3_EMR_POS)
}

/* (UDDRC_INIT3) DDR2: Value to write to MR register.
* Bit 8 is for DLL and the setting here is ignored.
* The uMCTL2 sets this bit appropriately.
* DDR3/DDR4: Value loaded into MR0 register.
* mDDR: Value to write to MR register.
* LPDDR2/LPDDR3/LPDDR4 - Value to write to MR1 register
*/
pub const UDDRC_INIT3_MR_POS:u32 = 16;
pub const UDDRC_INIT3_MR_MSK:u32 = 0xffffu32 << UDDRC_INIT3_MR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT3_MR(value:u32) -> u32 {
    UDDRC_INIT3_MR_MSK & ((value) << UDDRC_INIT3_MR_POS)
}

/* -------- UDDRC_INIT4 : (UDDRC_REGS Offset: 0xE0) SDRAM Initialization Register 4 -------- */
/* (UDDRC_INIT4) DDR2: Value to write to EMR3 register.
* DDR3/DDR4: Value to write to MR3 register
* mDDR/LPDDR2/LPDDR3: Unused
* LPDDR4: Value to write to MR13 register
*/
pub const UDDRC_INIT4_EMR3_POS:u32 = 0;
pub const UDDRC_INIT4_EMR3_MSK:u32 = 0xffffu32 << UDDRC_INIT4_EMR3_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT4_EMR3(value:u32) -> u32 {
    UDDRC_INIT4_EMR3_MSK & ((value) << UDDRC_INIT4_EMR3_POS)
}

/* (UDDRC_INIT4) DDR2: Value to write to EMR2 register.
* DDR3/DDR4: Value to write to MR2 register
* LPDDR2/LPDDR3/LPDDR4: Value to write to MR3 register
* mDDR: Unused
*/
pub const UDDRC_INIT4_EMR2_POS:u32 = 16;
pub const UDDRC_INIT4_EMR2_MSK:u32 = 0xffffu32 << UDDRC_INIT4_EMR2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT4_EMR2(value:u32) -> u32 {
    UDDRC_INIT4_EMR2_MSK & ((value) << UDDRC_INIT4_EMR2_POS)
}

/* -------- UDDRC_INIT5 : (UDDRC_REGS Offset: 0xE4)
* SDRAM Initialization Register 5 --------
*/
/* (UDDRC_INIT5) Maximum duration of the auto initialization, tINIT5.
* Present only in designs configured to support LPDDR2/LPDDR3.
* LPDDR2/LPDDR3 typically requires 10 us.
* Unit: 1024 DFI clock cycles.
*/
pub const UDDRC_INIT5_MAX_AUTO_INIT_X1024_POS:u32 = 0;
pub const UDDRC_INIT5_MAX_AUTO_INIT_X1024_MSK:u32 = 0x3ffu32 << UDDRC_INIT5_MAX_AUTO_INIT_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT5_MAX_AUTO_INIT_X1024 (value:u32) -> u32 {
    UDDRC_INIT5_MAX_AUTO_INIT_X1024_MSK & ((value) << UDDRC_INIT5_MAX_AUTO_INIT_X1024_POS)
}

/* (UDDRC_INIT5) ZQ initial calibration, tZQINIT.
* Present only in designs configured to support DDR3 or DDR4 or LPDDR2/LPDDR3.
* DDR3 typically requires 512 SDRAM clock cycles.
* DDR4 requires 1024 SDRAM clock cycles.
* LPDDR2/LPDDR3 requires 1 us.
* When the controller is operating in 1:2 frequency ratio mode, program this
* to JEDEC spec value divided by 2, and round it up to the next integer value.
* Unit: 32 DFI clock cycles.
*/
pub const UDDRC_INIT5_DEV_ZQINIT_X32_POS:u32 = 16;
pub const UDDRC_INIT5_DEV_ZQINIT_X32_MSK:u32 = 0xffu32 << UDDRC_INIT5_DEV_ZQINIT_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT5_DEV_ZQINIT_X32 (value:u32) -> u32 {
    UDDRC_INIT5_DEV_ZQINIT_X32_MSK & ((value) << UDDRC_INIT5_DEV_ZQINIT_X32_POS)
}

pub const UDDRC_INIT6_MR5_POS:u32 = 0;
pub const UDDRC_INIT6_MR5_MSK:u32 = 0xffffu32 << UDDRC_INIT6_MR5_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT6_MR5 (value:u32) -> u32 {
    UDDRC_INIT6_MR5_MSK & ((value) << UDDRC_INIT6_MR5_POS)
}

pub const UDDRC_INIT6_MR4_POS:u32 = 16;
pub const UDDRC_INIT6_MR4_MSK:u32 = 0xffffu32 << UDDRC_INIT6_MR4_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT6_MR4 (value:u32) -> u32 {
    UDDRC_INIT6_MR4_MSK & ((value) << UDDRC_INIT6_MR4_POS)
}

pub const UDDRC_INIT7_MR6_POS:u32 = 0;
pub const UDDRC_INIT7_MR6_MSK:u32 = 0xffffu32 << UDDRC_INIT7_MR6_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT7_MR6 (value:u32) -> u32 {
    UDDRC_INIT7_MR6_MSK & ((value) << UDDRC_INIT7_MR6_POS)
}

pub const UDDRC_INIT7_MR22_POS:u32 = 16;
pub const UDDRC_INIT7_MR22_MSK:u32 = 0xffffu32 << UDDRC_INIT7_MR22_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_INIT7_MR22 (value:u32) -> u32 {
    UDDRC_INIT7_MR22_MSK & ((value) << UDDRC_INIT7_MR22_POS)
}

/* -------- UDDRC_DIMMCTL : (UDDRC_REGS Offset: 0xF0)
* DIMM Control Register --------
*/
/* (UDDRC_DIMMCTL) Staggering enable for multi-rank accesses
* (for multi-rank UDIMM, RDIMM and LRDIMM implementations only).
* This is not supported for mDDR, LPDDR2, LPDDR3 or LPDDR4 SDRAMs.
* Note: Even if this bit is set it does not take care of software driven
* MR commands (via MRCTRL0/MRCTRL1), where software is responsible to
* send them to seperate ranks as appropriate.
- 1 - (DDR4) Send MRS commands to each ranks seperately
- 1 - (non-DDR4) Send all commands to even and odd ranks seperately
- 0 - Do not stagger accesses
*/
pub const UDDRC_DIMMCTL_DIMM_STAGGER_CS_EN:u32 = 0x1u32 << 0;

/* (UDDRC_DIMMCTL) Address Mirroring Enable
* (for multi-rank UDIMM implementations and multi-rank DDR4
* RDIMM/LRDIMM implementations).
* Some UDIMMs and DDR4 RDIMMs/LRDIMMs implement address mirroring for
* odd ranks, which means that the following address, bank address and
* bank group bits are swapped: (A3, A4), (A5, A6), (A7, A8), (BA0, BA1)
* and also (A11, A13), (BG0, BG1) for the DDR4. Setting this bit ensures that,
* for mode register accesses during the automatic initialization routine,
* these bits are swapped within the uMCTL2 to compensate for this
* UDIMM/RDIMM/LRDIMM swapping. In addition to the automatic initialization
* routine, in case of DDR4 UDIMM/RDIMM/LRDIMM, they are swapped during
* the automatic MRS access to enable/disable of a particular DDR4 feature.
* Note: This has no effect on the address of any other memory accesses,
* or of software-driven mode register accesses.
* This is not supported for mDDR, LPDDR2, LPDDR3 or LPDDR4 SDRAMs.
* Note: In case of x16 DDR4 DIMMs, BG1 output of MRS for the odd ranks is same
* as BG0 because BG1 is invalid, hence dimm_dis_bg_mirroring register
* must be set to 1.
- 1 - For odd ranks, implement address mirroring for MRS commands to
during initialization and for any automatic DDR4 MRS commands
(to be used if UDIMM/RDIMM/LRDIMM implements address mirroring)
- 0 - Do not implement address mirroring
*/
pub const UDDRC_DIMMCTL_DIMM_ADDR_MIRR_EN:u32 = 0x1u32 << 1;

/* -------- UDDRC_DRAMTMG0 : (UDDRC_REGS Offset: 0x100)
* SDRAM Timing Register 0 --------
*/
/* (UDDRC_DRAMTMG0) tRAS(min):  Minimum time between activate and
* precharge to the same bank.
* When the controller is operating in 1:2 frequency mode, 1T mode,
* program this to tRAS(min)/2. No rounding up.
* When the controller is operating in 1:2 frequency ratio mode, 2T mode
* or LPDDR4 mode, program this to (tRAS(min)/2) and round it up to the next
* integer value.
* Unit: Clocks
*/
pub const UDDRC_DRAMTMG0_T_RAS_MIN_POS:u32 = 0;
pub const UDDRC_DRAMTMG0_T_RAS_MIN_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG0_T_RAS_MIN_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG0_T_RAS_MIN (value:u32) -> u32 {
    UDDRC_DRAMTMG0_T_RAS_MIN_MSK & ((value) << UDDRC_DRAMTMG0_T_RAS_MIN_POS)
}

/* (UDDRC_DRAMTMG0) tRAS(max):  Maximum time between activate and precharge
* to same bank. This is the maximum time that a page can be kept open
* Minimum value of this register is 1. Zero is invalid.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to (tRAS(max)-1)/2. No rounding up.
* Unit: Multiples of 1024 clocks.
*/
pub const UDDRC_DRAMTMG0_T_RAS_MAX_POS:u32 = 8;
pub const UDDRC_DRAMTMG0_T_RAS_MAX_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG0_T_RAS_MAX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG0_T_RAS_MAX (value:u32) -> u32 {
    UDDRC_DRAMTMG0_T_RAS_MAX_MSK & ((value) << UDDRC_DRAMTMG0_T_RAS_MAX_POS)
}

/* (UDDRC_DRAMTMG0) tFAW Valid only when 8 or more banks(or banks x bank groups)
* are present.
* In 8-bank design, at most 4 banks must be activated in a rolling window
* of tFAW cycles.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to (tFAW/2) and round up to next integer value.
* In a 4-bank design, set this register to 0x1 independent of
* the 1:1/1:2 frequency mode.
Unit: Clocks
*/
pub const UDDRC_DRAMTMG0_T_FAW_POS:u32 = 16;
pub const UDDRC_DRAMTMG0_T_FAW_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG0_T_FAW_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG0_T_FAW (value:u32) -> u32 {
    UDDRC_DRAMTMG0_T_FAW_MSK & ((value) << UDDRC_DRAMTMG0_T_FAW_POS)
}

/* (UDDRC_DRAMTMG0) Minimum time between write and precharge to same bank.
* Unit: Clocks
* Specifications: WL + BL/2 + tWR = approximately 8 cycles + 15 ns =
* 14 clocks @400MHz and less for lower frequencies
* where:
- WL = write latency
- BL = burst length. This must match the value programmed in the BL bit of the
mode register to the SDRAM.
BST (burst terminate) is not supported at present.
- tWR = Write recovery time. This comes directly from the SDRAM specification.
* Add one extra cycle for LPDDR2/LPDDR3/LPDDR4 for this parameter.
*
* When the controller is operating in 1:2 frequency ratio mode, 1T mode,
* divide the above value by 2. No rounding up.
* When the controller is operating in 1:2 frequency ratio mode, 2T mode
* or LPDDR4 mode, divide the above value by 2 and round it up to the
* next integer value.
*
* Note that, depending on the PHY, if using LRDIMM, it may be necessary
* to adjust the value of this parameter to compensate for the extra cycle
* of latency through the LRDIMM.
*/
pub const UDDRC_DRAMTMG0_WR2PRE_POS:u32 = 24;
pub const UDDRC_DRAMTMG0_WR2PRE_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG0_WR2PRE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG0_WR2PRE (value:u32) -> u32 {
    UDDRC_DRAMTMG0_WR2PRE_MSK & ((value) << UDDRC_DRAMTMG0_WR2PRE_POS)
}

/* -------- UDDRC_DRAMTMG1 : (UDDRC_REGS Offset: 0x104)
* SDRAM Timing Register 1 --------
*/
/* (UDDRC_DRAMTMG1) tRC:  Minimum time between activates to same bank.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to (tRC/2) and round up to next integer value.
* Unit: Clocks.
*/
pub const UDDRC_DRAMTMG1_T_RC_POS:u32 = 0;
pub const UDDRC_DRAMTMG1_T_RC_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG1_T_RC_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG1_T_RC(value:u32) -> u32 {
    UDDRC_DRAMTMG1_T_RC_MSK & ((value) << UDDRC_DRAMTMG1_T_RC_POS)
}

/* (UDDRC_DRAMTMG1) tRTP:  Minimum time from read to precharge of same bank.
- DDR2: tAL + BL/2 + max(tRTP, 2) - 2
- DDR3: tAL + max (tRTP, 4)
- DDR4: Max of following two equations:
tAL + max (tRTP, 4) or,
RL + BL/2 - tRP (*).
- mDDR: BL/2
- LPDDR2: Depends on if it's LPDDR2-S2 or LPDDR2-S4:
LPDDR2-S2: BL/2 + tRTP - 1.
LPDDR2-S4: BL/2 + max(tRTP,2) - 2.
- LPDDR3: BL/2 +  max(tRTP,4) - 4
- LPDDR4: BL/2 + max(tRTP,8) - 8
(*) When both DDR4 SDRAM and ST-MRAM are used simultaneously,
* use SDRAM's tRP value for calculation.
*
* When the controller is operating in 1:2 mode, 1T mode, divide the above
* value by 2. No rounding up.
* When the controller is operating in 1:2 mode, 2T mode or LPDDR4 mode,
* divide the above value by 2 and round it up to the next integer value.
* Unit: Clocks.
*/
pub const UDDRC_DRAMTMG1_RD2PRE_POS:u32 = 8;
pub const UDDRC_DRAMTMG1_RD2PRE_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG1_RD2PRE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG1_RD2PRE(value:u32) -> u32 {
    UDDRC_DRAMTMG1_RD2PRE_MSK & ((value) << UDDRC_DRAMTMG1_RD2PRE_POS)
}

/* (UDDRC_DRAMTMG1) tXP: Minimum time after power-down exit to any operation.
* For DDR3, this should be programmed to tXPDLL if slow powerdown exit
* is selected in MR0[12].
* If C/A parity for DDR4 is used, set to (tXP+PL) instead.
* If LPDDR4 is selected and its spec has tCKELPD parameter,
* set to the larger of tXP and tCKELPD instead.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to (tXP/2) and round it up to the next integer value.
* Units: Clocks
*/
pub const UDDRC_DRAMTMG1_T_XP_POS:u32 = 16;
pub const UDDRC_DRAMTMG1_T_XP_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG1_T_XP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG1_T_XP(value:u32) -> u32 {
    UDDRC_DRAMTMG1_T_XP_MSK & ((value) << UDDRC_DRAMTMG1_T_XP_POS)
}

/* -------- UDDRC_DRAMTMG2 : (UDDRC_REGS Offset: 0x108)
* SDRAM Timing Register 2 --------
*/

/* (UDDRC_DRAMTMG2)
* DDR4: CWL + PL + BL/2 + tWTR_L
* LPDDR2/3/4: WL + BL/2 + tWTR + 1
* Others: CWL + BL/2 + tWTR
* In DDR4, minimum time from write command to read command for same bank group.
* In others, minimum time from write command to read command. Includes time for
* bus turnaround, recovery times, and all per-bank, per-rank,
* and global constraints.
* Unit: Clocks.
*Where:
- CWL = CAS write latency
- WL = Write latency
- PL = Parity latency
- BL = burst length. This must match the value programmed in the BL bit of the
mode register to the SDRAM
- tWTR_L = internal write to read command delay for same bank group.
This comes directly from the SDRAM specification.
- tWTR = internal write to read command delay.
This comes directly from the SDRAM specification.
* Add one extra cycle for LPDDR2/LPDDR3/LPDDR4 operation.
* When the controller is operating in 1:2 mode, divide the value calculated
* using the above equation by 2, and round it up to next integer.
*/
pub const UDDRC_DRAMTMG2_WR2RD_POS:u32 = 0;
pub const UDDRC_DRAMTMG2_WR2RD_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG2_WR2RD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG2_WR2RD(value:u32) -> u32 {
    UDDRC_DRAMTMG2_WR2RD_MSK & ((value) << UDDRC_DRAMTMG2_WR2RD_POS)
}

/* (UDDRC_DRAMTMG2)
* DDR2/3/mDDR: RL + BL/2 + 2 - WL
* DDR4: RL + BL/2 + 1 + WR_PREAMBLE - WL
* LPDDR2/LPDDR3: RL + BL/2 + RU(tDQSCKmax/tCK) + 1 - WL
* LPDDR4(DQ ODT is Disabled): RL + BL/2 + RU(tDQSCKmax/tCK) +
WR_PREAMBLE + RD_POSTAMBLE - WL
* LPDDR4(DQ ODT is Enabled) : RL + BL/2 + RU(tDQSCKmax/tCK) +
RD_POSTAMBLE - ODTLon - RU(tODTon(min)/tCK)
*
* Minimum time from read command to write command. Include time for bus
* turnaround and all per-bank, per-rank, and global constraints.
* Please see the relevant PHY databook for details of what should be
* included here.
* Unit: Clocks.
* Where:
- WL = write latency
- BL = burst length. This must match the value programmed in the BL bit of
the mode register to the SDRAM
- RL = read latency = CAS latency
- WR_PREAMBLE = write preamble. This is unique to DDR4 and LPDDR4.
- RD_POSTAMBLE = read postamble. This is unique to LPDDR4.
* For LPDDR2/LPDDR3/LPDDR4, if derating is enabled (DERATEEN.derate_enable=1),
* derated tDQSCKmax should be used.
*
* When the controller is operating in 1:2 frequency ratio mode,
* divide the value calculated using the above equation by 2,
* and round it up to next integer.
*
* Note that, depending on the PHY, if using LRDIMM, it may be necessary to
* adjust the value of this parameter to compensate for the extra cycle of
* latency through the LRDIMM.
*/
pub const UDDRC_DRAMTMG2_RD2WR_POS:u32 = 8;
pub const UDDRC_DRAMTMG2_RD2WR_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG2_RD2WR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG2_RD2WR(value:u32) -> u32 {
    UDDRC_DRAMTMG2_RD2WR_MSK & ((value) << UDDRC_DRAMTMG2_RD2WR_POS)
}

/* (UDDRC_DRAMTMG2) Set to RL
* Time from read command to read data on SDRAM interface.
* This must be set to RL.
* Note that, depending on the PHY, if using RDIMM/LRDIMM, it may be necessary
* to adjust the value of RL to compensate for the extra cycle of latency
* through the RDIMM/LRDIMM.
* When the controller is operating in 1:2 frequency ratio mode, divide the
* value calculated using the above equation by 2, and round it up
* to next integer.
* This register field is not required for DDR2 and DDR3 (except if
* MEMC_TRAINING is set), as the DFI read and write latencies defined
* in DFITMG0 and DFITMG1 are sufficient for those protocols
* Unit: clocks
*/
pub const UDDRC_DRAMTMG2_READ_LATENCY_POS:u32 = 16;
pub const UDDRC_DRAMTMG2_READ_LATENCY_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG2_READ_LATENCY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG2_READ_LATENCY (value:u32) -> u32 {
    UDDRC_DRAMTMG2_READ_LATENCY_MSK & ((value) << UDDRC_DRAMTMG2_READ_LATENCY_POS)
}

/* (UDDRC_DRAMTMG2) Set to WL
* Time from write command to write data on SDRAM interface.
* This must be set to WL.
* For mDDR, it should normally be set to 1.
* Note that, depending on the PHY, if using RDIMM/LRDIMM, it may be necessary
* to adjust the value of WL to compensate for the extra cycle of latency
* through the RDIMM/LRDIMM.
* When the controller is operating in 1:2 frequency ratio mode, divide the
* value calculated using the above equation by 2, and round it up to
* next integer.
* This register field is not required for DDR2 and DDR3 (except if
* MEMC_TRAINING is set), as the DFI read and write latencies defined in
* DFITMG0 and DFITMG1 are sufficient for those protocols
* Unit: clocks
*/
pub const UDDRC_DRAMTMG2_WRITE_LATENCY_POS:u32 = 24;
pub const UDDRC_DRAMTMG2_WRITE_LATENCY_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG2_WRITE_LATENCY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG2_WRITE_LATENCY (value:u32) -> u32 {
    UDDRC_DRAMTMG2_WRITE_LATENCY_MSK & ((value) << UDDRC_DRAMTMG2_WRITE_LATENCY_POS)
}

/* -------- UDDRC_DRAMTMG3 : (UDDRC_REGS Offset: 0x10C)
* SDRAM Timing Register 3 --------
*/
/* (UDDRC_DRAMTMG3) tMOD: Parameter used only in DDR3 and DDR4.
* Cycles between load mode command and following non-load mode command.
* If C/A parity for DDR4 is used, set to tMOD_PAR(tMOD+PL) instead.
* If MPR writes for DDR4 are used, set to tMOD + AL
* (or tMPD_PAR + AL if C/A parity is also used).
* Set to tMOD if controller is operating in 1:1 frequency ratio mode,
* or tMOD/2 (rounded up to next integer) if controller is operating in
* 1:2 frequency ratio mode.
* Note that if using RDIMM/LRDIMM, depending on the PHY, it may be necessary
* to adjust the value of this parameter to compensate for the extra cycle of
* latency applied to mode register writes by the RDIMM/LRDIMM chip.
* Also note that if using LRDIMM, the minimum value of this register is
* tMRD_L2 if controller is operating in 1:1 frequency ratio mode,
* or tMRD_L2/2 (rounded up to next integer) if controller is operating in
* 1:2 frequency ratio mode.
*/
pub const UDDRC_DRAMTMG3_T_MOD_POS:u32 = 0;
pub const UDDRC_DRAMTMG3_T_MOD_MSK:u32 = 0x3ffu32 << UDDRC_DRAMTMG3_T_MOD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG3_T_MOD(value:u32) -> u32 {
    UDDRC_DRAMTMG3_T_MOD_MSK & ((value) << UDDRC_DRAMTMG3_T_MOD_POS)
}

/* (UDDRC_DRAMTMG3) tMRD: Cycles to wait after a mode register write or read.
* Depending on the connected SDRAM, tMRD represents:
* DDR2/mDDR: Time from MRS to any command
* DDR3/4: Time from MRS to MRS command
* LPDDR2: not used
* LPDDR3/4: Time from MRS to non-MRS command.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* (tMRD/2) and round it up to the next integer value.
* If C/A parity for DDR4 is used, set to tMRD_PAR(tMOD+PL) instead.
*/
pub const UDDRC_DRAMTMG3_T_MRD_POS:u32 = 12;
pub const UDDRC_DRAMTMG3_T_MRD_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG3_T_MRD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG3_T_MRD(value:u32) -> u32 {
    UDDRC_DRAMTMG3_T_MRD_MSK & ((value) << UDDRC_DRAMTMG3_T_MRD_POS)
}

/* (UDDRC_DRAMTMG3) Time to wait after a mode register write or read (MRW or MRR).
* Present only in designs configured to support LPDDR2, LPDDR3 or LPDDR4.
* LPDDR2 typically requires value of 5.
* LPDDR3 typically requires value of 10.
* LPDDR4: Set this to the larger of tMRW and tMRWCKEL.
* For LPDDR2, this register is used for the time from a MRW/MRR
* to all other commands.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to the above values divided by 2 and round it up
* to the next integer value.
* For LDPDR3, this register is used for the time from a MRW/MRR to a MRW/MRR.
*/
pub const UDDRC_DRAMTMG3_T_MRW_POS:u32 = 20;
pub const UDDRC_DRAMTMG3_T_MRW_MSK:u32 = 0x3ffu32 << UDDRC_DRAMTMG3_T_MRW_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG3_T_MRW(value:u32) -> u32 {
    UDDRC_DRAMTMG3_T_MRW_MSK & ((value) << UDDRC_DRAMTMG3_T_MRW_POS)
}


/* -------- UDDRC_DRAMTMG4 : (UDDRC_REGS Offset: 0x110)
* SDRAM Timing Register 4 --------
*/
/* (UDDRC_DRAMTMG4) tRP:  Minimum time from precharge to activate of same bank.
* When the controller is operating in 1:1 frequency ratio mode, t_rp should be
* set to RoundUp(tRP/tCK).
* When the controller is operating in 1:2 frequency ratio mode, t_rp should be
* set to RoundDown(RoundUp(tRP/tCK)/2) + 1.
* When the controller is operating in 1:2 frequency ratio mode in LPDDR4, t_rp
* should be set to RoundUp(RoundUp(tRP/tCK)/2).
* Unit: Clocks.
*/
pub const UDDRC_DRAMTMG4_T_RP_POS:u32 = 0;
pub const UDDRC_DRAMTMG4_T_RP_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG4_T_RP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG4_T_RP(value:u32) -> u32 {
    UDDRC_DRAMTMG4_T_RP_MSK & ((value) << UDDRC_DRAMTMG4_T_RP_POS)
}

/* (UDDRC_DRAMTMG4) DDR4: tRRD_L: Minimum time between activates from
* bank "a" to bank "b" for same bank group.
* Others: tRRD: Minimum time between activates from bank "a" to bank "b"
* When the controller is operating in 1:2 frequency ratio mode, program this
* to (tRRD_L/2 or tRRD/2) and round it up to the next integer value.
* Unit: Clocks.
*/
pub const UDDRC_DRAMTMG4_T_RRD_POS:u32 = 8;
pub const UDDRC_DRAMTMG4_T_RRD_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG4_T_RRD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG4_T_RRD(value:u32) -> u32 {
    UDDRC_DRAMTMG4_T_RRD_MSK & ((value) << UDDRC_DRAMTMG4_T_RRD_POS)
}

/* (UDDRC_DRAMTMG4) DDR4: tCCD_L: This is the minimum time between
* two reads or two writes for same bank group.
* Others: tCCD: This is the minimum time between two reads or two writes.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* (tCCD_L/2 or tCCD/2) and round it up to the next integer value.
* Unit: clocks.
*/
pub const UDDRC_DRAMTMG4_T_CCD_POS:u32 = 16;
pub const UDDRC_DRAMTMG4_T_CCD_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG4_T_CCD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG4_T_CCD(value:u32) -> u32 {
    UDDRC_DRAMTMG4_T_CCD_MSK & ((value) << UDDRC_DRAMTMG4_T_CCD_POS)
}

/* (UDDRC_DRAMTMG4) tRCD - tAL: Minimum time from activate to read or write
* command to same bank.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* ((tRCD - tAL)/2) and round it up to the next integer value.
* Minimum value allowed for this register is 1, which implies minimum
* (tRCD - tAL) value to be 2 when the controller is operating in 1:2 frequency
* ratio mode.
*Unit: Clocks.
*/
pub const UDDRC_DRAMTMG4_T_RCD_POS:u32 = 24;
pub const UDDRC_DRAMTMG4_T_RCD_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG4_T_RCD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG4_T_RCD(value:u32) -> u32 {
    UDDRC_DRAMTMG4_T_RCD_MSK & ((value) << UDDRC_DRAMTMG4_T_RCD_POS)
}

/* -------- UDDRC_DRAMTMG5 : (UDDRC_REGS Offset: 0x114)
SDRAM Timing Register 5 --------
*/
/* (UDDRC_DRAMTMG5) Minimum number of cycles of CKE HIGH/LOW during
* power-down and self refresh.
- LPDDR2/LPDDR3 mode: Set this to the larger of tCKE or tCKESR
- LPDDR4 mode: Set this to the larger of tCKE or tSR.
- Non-LPDDR2/non-LPDDR3/non-LPDDR4 designs: Set this to tCKE value.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* (value described above)/2 and round it up to the next integer value.
* Unit: Clocks.
*/
pub const UDDRC_DRAMTMG5_T_CKE_POS:u32 = 0;
pub const UDDRC_DRAMTMG5_T_CKE_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG5_T_CKE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG5_T_CKE(value:u32) -> u32 {
    UDDRC_DRAMTMG5_T_CKE_MSK & ((value) << UDDRC_DRAMTMG5_T_CKE_POS)
}

/* (UDDRC_DRAMTMG5) Minimum CKE low width for Self refresh or Self refresh
* power down entry to exit timing in memory clock cycles.
* Recommended settings:
- mDDR: tRFC
- LPDDR2: tCKESR
- LPDDR3: tCKESR
- LPDDR4: max(tCKE, tSR)
- DDR2: tCKE
- DDR3: tCKE + 1
- DDR4: tCKE + 1 (+ PL(parity latency)(*))
* (*)Only if CRCPARCTL1.caparity_disable_before_sr=0,
* this register should be increased by PL.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
*/
pub const UDDRC_DRAMTMG5_T_CKESR_POS:u32 = 8;
pub const UDDRC_DRAMTMG5_T_CKESR_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG5_T_CKESR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG5_T_CKESR(value:u32) -> u32 {
    UDDRC_DRAMTMG5_T_CKESR_MSK & ((value) << UDDRC_DRAMTMG5_T_CKESR_POS)
}

/* (UDDRC_DRAMTMG5) This is the time after Self Refresh Down Entry that CK is
* maintained as a valid clock. Specifies the clock disable delay after SRE.
* Recommended settings:
- mDDR: 0
- LPDDR2: 2
- LPDDR3: 2
- LPDDR4: tCKELCK
- DDR2: 1
- DDR3: max (10 ns, 5 tCK)
- DDR4: max (10 ns, 5 tCK) (+ PL(parity latency)(*))
*
* (*)Only if CRCPARCTL1.caparity_disable_before_sr=0,
* this register should be increased by PL.
* When the controller is operating in 1:2 frequency ratio mode,
* program this to recommended value divided by two and round it up to
* next integer.
*/
pub const UDDRC_DRAMTMG5_T_CKSRE_POS:u32 = 16;
pub const UDDRC_DRAMTMG5_T_CKSRE_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG5_T_CKSRE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG5_T_CKSRE(value:u32) -> u32 {
    UDDRC_DRAMTMG5_T_CKSRE_MSK & ((value) << UDDRC_DRAMTMG5_T_CKSRE_POS)
}

/* (UDDRC_DRAMTMG5) This is the time before Self Refresh Exit that CK
* is maintained as a valid clock before issuing SRX.
* Specifies the clock stable time before SRX.
* Recommended settings:
- mDDR: 1
- LPDDR2: 2
- LPDDR3: 2
- LPDDR4: tCKCKEH
- DDR2: 1
- DDR3: tCKSRX
- DDR4: tCKSRX
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
*/
pub const UDDRC_DRAMTMG5_T_CKSRX_POS:u32 = 24;
pub const UDDRC_DRAMTMG5_T_CKSRX_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG5_T_CKSRX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG5_T_CKSRX(value:u32) -> u32 {
    UDDRC_DRAMTMG5_T_CKSRX_MSK & ((value) << UDDRC_DRAMTMG5_T_CKSRX_POS)
}

/* -------- UDDRC_DRAMTMG6 : (UDDRC_REGS Offset: 0x118)
* SDRAM Timing Register 6 --------
*/
/* (UDDRC_DRAMTMG6) This is the time before Clock Stop Exit that CK is
* maintained as a valid clock before issuing Clock Stop Exit.
* Specifies the clock stable time before next command after Clock Stop Exit.
* Recommended settings:
- mDDR: 1
- LPDDR2: tXP + 2
- LPDDR3: tXP + 2
- LPDDR4: tXP + 2
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3/LPDDR4
* devices.
*/
pub const UDDRC_DRAMTMG6_T_CKCSX_POS:u32 = 0;
pub const UDDRC_DRAMTMG6_T_CKCSX_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG6_T_CKCSX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG6_T_CKCSX(value:u32) -> u32 {
    UDDRC_DRAMTMG6_T_CKCSX_MSK & ((value) << UDDRC_DRAMTMG6_T_CKCSX_POS)
}

/* (UDDRC_DRAMTMG6) This is the time before Deep Power Down Exit that CK is
* maintained as a valid clock before issuing DPDX.
* Specifies the clock stable time before DPDX.
* Recommended settings:
- mDDR: 1
- LPDDR2: 2
- LPDDR3: 2
*
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* This is only present for designs supporting mDDR or LPDDR2 devices.
*/
pub const UDDRC_DRAMTMG6_T_CKDPDX_POS:u32 = 16;
pub const UDDRC_DRAMTMG6_T_CKDPDX_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG6_T_CKDPDX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG6_T_CKDPDX(value:u32) -> u32 {
    UDDRC_DRAMTMG6_T_CKDPDX_MSK & ((value) << UDDRC_DRAMTMG6_T_CKDPDX_POS)
}

/* (UDDRC_DRAMTMG6) This is the time after Deep Power Down Entry that CK
* is maintained as a valid clock. Specifies the clock disable delay after DPDE. 
* Recommended settings:
- mDDR: 0
- LPDDR2: 2
- LPDDR3: 2
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3 devices.
*/
pub const UDDRC_DRAMTMG6_T_CKDPDE_POS:u32 = 24;
pub const UDDRC_DRAMTMG6_T_CKDPDE_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG6_T_CKDPDE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG6_T_CKDPDE(value:u32) -> u32 {
    UDDRC_DRAMTMG6_T_CKDPDE_MSK & ((value) << UDDRC_DRAMTMG6_T_CKDPDE_POS)
}

/* -------- UDDRC_DRAMTMG7 : (UDDRC_REGS Offset: 0x11C)
* SDRAM Timing Register 7 --------
*/
/* (UDDRC_DRAMTMG7) This is the time before Power Down Exit that CK
* is maintained as a valid clock before issuing PDX.
* Specifies the clock stable time before PDX.
* Recommended settings:
- mDDR: 0
- LPDDR2: 2
- LPDDR3: 2
- LPDDR4: 2
* When using DDR2/3/4 SDRAM, this register should be set to the same value
* as DRAMTMG5.t_cksrx.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_DRAMTMG7_T_CKPDX_POS:u32 = 0;
pub const UDDRC_DRAMTMG7_T_CKPDX_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG7_T_CKPDX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG7_T_CKPDX(value:u32) -> u32 {
    UDDRC_DRAMTMG7_T_CKPDX_MSK & ((value) << UDDRC_DRAMTMG7_T_CKPDX_POS)
}

/* (UDDRC_DRAMTMG7) This is the time after Power Down Entry that CK is
* maintained as a valid clock. Specifies the clock disable delay after PDE.
* Recommended settings:
- mDDR: 0
- LPDDR2: 2
- LPDDR3: 2
- LPDDR4: tCKELCK
* When using DDR2/3/4 SDRAM, this register should be set to the same value
* as DRAMTMG5.t_cksre.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3/LPDDR4
* devices.
*/
pub const UDDRC_DRAMTMG7_T_CKPDE_POS:u32 = 8;
pub const UDDRC_DRAMTMG7_T_CKPDE_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG7_T_CKPDE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG7_T_CKPDE(value:u32) -> u32 {
    UDDRC_DRAMTMG7_T_CKPDE_MSK & ((value) << UDDRC_DRAMTMG7_T_CKPDE_POS)
}

/* -------- UDDRC_DRAMTMG8 : (UDDRC_REGS Offset: 0x120)
* SDRAM Timing Register 8 --------
*/
/* (UDDRC_DRAMTMG8) tXS: Exit Self Refresh to commands not requiring a
* locked DLL.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* the above value divided by 2 and round up to next integer value.
* Unit: Multiples of 32 clocks.
* Note: Used only for DDR2, DDR3 and DDR4 SDRAMs.
*/
pub const UDDRC_DRAMTMG8_T_XS_X32_POS:u32 = 0;
pub const UDDRC_DRAMTMG8_T_XS_X32_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG8_T_XS_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG8_T_XS_X32(value:u32) -> u32 {
    UDDRC_DRAMTMG8_T_XS_X32_MSK & ((value) << UDDRC_DRAMTMG8_T_XS_X32_POS)
}

/* (UDDRC_DRAMTMG8) tXSDLL: Exit Self Refresh to commands requiring a locked DLL.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* the above value divided by 2 and round up to next integer value.
* Unit: Multiples of 32 clocks.
* Note: Used only for DDR2, DDR3 and DDR4 SDRAMs.
*/
pub const UDDRC_DRAMTMG8_T_XS_DLL_X32_POS:u32 = 8;
pub const UDDRC_DRAMTMG8_T_XS_DLL_X32_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG8_T_XS_DLL_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG8_T_XS_DLL_X32 (value:u32) -> u32 {
    UDDRC_DRAMTMG8_T_XS_DLL_X32_MSK & ((value) << UDDRC_DRAMTMG8_T_XS_DLL_X32_POS)
}

pub const UDDRC_DRAMTMG8_T_XS_ABORT_X32_POS:u32 = 16;
pub const UDDRC_DRAMTMG8_T_XS_ABORT_X32_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG8_T_XS_ABORT_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG8_T_XS_ABORT_X32 (value:u32) -> u32 {
    UDDRC_DRAMTMG8_T_XS_ABORT_X32_MSK & ((value) << UDDRC_DRAMTMG8_T_XS_ABORT_X32_POS)
}

pub const UDDRC_DRAMTMG8_T_XS_FAST_X32_POS:u32 = 24;
pub const UDDRC_DRAMTMG8_T_XS_FAST_X32_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG8_T_XS_FAST_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG8_T_XS_FAST_X32 (value:u32) -> u32 {
    UDDRC_DRAMTMG8_T_XS_FAST_X32_MSK & ((value) << UDDRC_DRAMTMG8_T_XS_FAST_X32_POS)
}

pub const UDDRC_DRAMTMG9_WR2RD_S_POS:u32 = 0;
pub const UDDRC_DRAMTMG9_WR2RD_S_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG9_WR2RD_S_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG9_WR2RD_S(value:u32) -> u32 {
    UDDRC_DRAMTMG9_WR2RD_S_MSK & ((value) << UDDRC_DRAMTMG9_WR2RD_S_POS)
}

pub const UDDRC_DRAMTMG9_T_RRD_S_POS:u32 = 8;
pub const UDDRC_DRAMTMG9_T_RRD_S_MSK:u32 = 0xfu32 << UDDRC_DRAMTMG9_T_RRD_S_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG9_T_RRD_S (value:u32) -> u32 {
    UDDRC_DRAMTMG9_T_RRD_S_MSK & ((value) << UDDRC_DRAMTMG9_T_RRD_S_POS)
}

pub const UDDRC_DRAMTMG9_T_CCD_S_POS:u32 = 16;
pub const UDDRC_DRAMTMG9_T_CCD_S_MSK:u32 = 0x7u32 << UDDRC_DRAMTMG9_T_CCD_S_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG9_T_CCD_S (value:u32) -> u32 {
    UDDRC_DRAMTMG9_T_CCD_S_MSK & ((value) << UDDRC_DRAMTMG9_T_CCD_S_POS)
}

pub const UDDRC_DRAMTMG9_DDR4_WR_PREAMBLE:u32 = 0x1u32 << 30;

pub const UDDRC_DRAMTMG10_T_GEAR_HOLD_POS:u32 = 0;
pub const UDDRC_DRAMTMG10_T_GEAR_HOLD_MSK:u32 = 0x3u32 << UDDRC_DRAMTMG10_T_GEAR_HOLD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG10_T_GEAR_HOLD(value:u32) -> u32 {
    UDDRC_DRAMTMG10_T_GEAR_HOLD_MSK & ((value) << UDDRC_DRAMTMG10_T_GEAR_HOLD_POS)
}

pub const UDDRC_DRAMTMG10_T_GEAR_SETUP_POS:u32 = 2;
pub const UDDRC_DRAMTMG10_T_GEAR_SETUP_MSK:u32 = 0x3u32 << UDDRC_DRAMTMG10_T_GEAR_SETUP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG10_T_GEAR_SETUP (value:u32) -> u32 {
    UDDRC_DRAMTMG10_T_GEAR_SETUP_MSK & ((value) << UDDRC_DRAMTMG10_T_GEAR_SETUP_POS)
}

pub const UDDRC_DRAMTMG10_T_CMD_GEAR_POS:u32 = 8;
pub const UDDRC_DRAMTMG10_T_CMD_GEAR_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG10_T_CMD_GEAR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG10_T_CMD_GEAR (value:u32) -> u32 {
    UDDRC_DRAMTMG10_T_CMD_GEAR_MSK & ((value) << UDDRC_DRAMTMG10_T_CMD_GEAR_POS)
}

pub const UDDRC_DRAMTMG10_T_SYNC_GEAR_POS:u32 = 16;
pub const UDDRC_DRAMTMG10_T_SYNC_GEAR_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG10_T_SYNC_GEAR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG10_T_SYNC_GEAR (value:u32) -> u32 {
    UDDRC_DRAMTMG10_T_SYNC_GEAR_MSK & ((value) << UDDRC_DRAMTMG10_T_SYNC_GEAR_POS)
}

pub const UDDRC_DRAMTMG11_T_CKMPE_POS:u32 = 0;
pub const UDDRC_DRAMTMG11_T_CKMPE_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG11_T_CKMPE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG11_T_CKMPE(value:u32) -> u32 {
    UDDRC_DRAMTMG11_T_CKMPE_MSK & ((value) << UDDRC_DRAMTMG11_T_CKMPE_POS)
}

pub const UDDRC_DRAMTMG11_T_MPX_S_POS:u32 = 8;
pub const UDDRC_DRAMTMG11_T_MPX_S_MSK:u32 = 0x3u32 << UDDRC_DRAMTMG11_T_MPX_S_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG11_T_MPX_S (value:u32) -> u32 {
    UDDRC_DRAMTMG11_T_MPX_S_MSK & ((value) << UDDRC_DRAMTMG11_T_MPX_S_POS)
}

pub const UDDRC_DRAMTMG11_T_MPX_LH_POS:u32 = 16;
pub const UDDRC_DRAMTMG11_T_MPX_LH_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG11_T_MPX_LH_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG11_T_MPX_LH (value:u32) -> u32 {
    UDDRC_DRAMTMG11_T_MPX_LH_MSK & ((value) << UDDRC_DRAMTMG11_T_MPX_LH_POS)
}

pub const UDDRC_DRAMTMG11_POST_MPSM_GAP_X32_POS:u32 = 24;
pub const UDDRC_DRAMTMG11_POST_MPSM_GAP_X32_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG11_POST_MPSM_GAP_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG11_POST_MPSM_GAP_X32 (value:u32) -> u32 {
    UDDRC_DRAMTMG11_POST_MPSM_GAP_X32_MSK & ((value) << UDDRC_DRAMTMG11_POST_MPSM_GAP_X32_POS)
}

pub const UDDRC_DRAMTMG12_T_MRD_PDA_POS:u32 = 0;
pub const UDDRC_DRAMTMG12_T_MRD_PDA_MSK:u32 = 0x1fu32 << UDDRC_DRAMTMG12_T_MRD_PDA_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG12_T_MRD_PDA(value:u32) -> u32 {
    UDDRC_DRAMTMG12_T_MRD_PDA_MSK & ((value) << UDDRC_DRAMTMG12_T_MRD_PDA_POS)
}

pub const UDDRC_DRAMTMG12_T_CMDCKE_POS:u32 = 16;
pub const UDDRC_DRAMTMG12_T_CMDCKE_MSK:u32 = 0x3u32 << UDDRC_DRAMTMG12_T_CMDCKE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG12_T_CMDCKE(value:u32) -> u32 {
    UDDRC_DRAMTMG12_T_CMDCKE_MSK & ((value) << UDDRC_DRAMTMG12_T_CMDCKE_POS)
}

pub const UDDRC_DRAMTMG12_T_WR_MPR_POS:u32 = 24;
pub const UDDRC_DRAMTMG12_T_WR_MPR_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG12_T_WR_MPR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG12_T_WR_MPR(value:u32) -> u32 {
    UDDRC_DRAMTMG12_T_WR_MPR_MSK & ((value) << UDDRC_DRAMTMG12_T_WR_MPR_POS)
}

pub const UDDRC_DRAMTMG13_T_PPD_POS:u32 = 0;
pub const UDDRC_DRAMTMG13_T_PPD_MSK:u32 = 0x7u32 << UDDRC_DRAMTMG13_T_PPD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG13_T_PPD(value:u32) -> u32 {
    UDDRC_DRAMTMG13_T_PPD_MSK & ((value) << UDDRC_DRAMTMG13_T_PPD_POS)
}

pub const UDDRC_DRAMTMG13_T_CCD_MW_POS:u32 = 16;
pub const UDDRC_DRAMTMG13_T_CCD_MW_MSK:u32 = 0x3fu32 << UDDRC_DRAMTMG13_T_CCD_MW_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG13_T_CCD_MW(value:u32) -> u32 {
    UDDRC_DRAMTMG13_T_CCD_MW_MSK & ((value) << UDDRC_DRAMTMG13_T_CCD_MW_POS)
}

pub const UDDRC_DRAMTMG13_ODTLOFF_POS:u32 = 24;
pub const UDDRC_DRAMTMG13_ODTLOFF_MSK:u32 = 0x7fu32 << UDDRC_DRAMTMG13_ODTLOFF_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG13_ODTLOFF(value:u32) -> u32 {
    UDDRC_DRAMTMG13_ODTLOFF_MSK & ((value) << UDDRC_DRAMTMG13_ODTLOFF_POS)
}


/* -------- UDDRC_DRAMTMG14 : (UDDRC_REGS Offset: 0x138)
* SDRAM Timing Register 14 --------
*/
/* (UDDRC_DRAMTMG14) tXSR: Exit Self Refresh to any command.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* the above value divided by 2 and round up to next integer value.
* Note: Used only for mDDR/LPDDR2/LPDDR3/LPDDR4 mode.
*/
pub const UDDRC_DRAMTMG14_T_XSR_POS:u32 = 0;
pub const UDDRC_DRAMTMG14_T_XSR_MSK:u32 = 0xfffu32 << UDDRC_DRAMTMG14_T_XSR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG14_T_XSR(value:u32) -> u32 {
    UDDRC_DRAMTMG14_T_XSR_MSK & ((value) << UDDRC_DRAMTMG14_T_XSR_POS)
}


/* -------- UDDRC_DRAMTMG15 : (UDDRC_REGS Offset: 0x13C)
* SDRAM Timing Register 15 --------
*/
/* (UDDRC_DRAMTMG15) tSTAB: Stabilization time.
* It is required in the following two cases for DDR3/DDR4 RDIMM : 
- when exiting power saving mode, if the clock was stopped, after re-enabling
it the clock must be stable for a time specified by tSTAB
- in the case of input clock frequency change (DDR4)
- after issuing control words that refers to clock timing
* (Specification: 6us for DDR3, 5us for DDR4)
*
* When the controller is operating in 1:2 frequency ratio mode, program this to
* recommended value divided by two and round it up to next integer.
* Unit: Multiples of 32 clock cycles.
*/
pub const UDDRC_DRAMTMG15_T_STAB_X32_POS:u32 = 0;
pub const UDDRC_DRAMTMG15_T_STAB_X32_MSK:u32 = 0xffu32 << UDDRC_DRAMTMG15_T_STAB_X32_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DRAMTMG15_T_STAB_X32 (value:u32) -> u32 {
    UDDRC_DRAMTMG15_T_STAB_X32_MSK & ((value) << UDDRC_DRAMTMG15_T_STAB_X32_POS)
}

/* (UDDRC_DRAMTMG15)
- 1 - Enable using tSTAB when exiting DFI LP. Needs to be set when the PHY is
stopping the clock during DFI LP to save maximum power.
- 0 - Disable using tSTAB when exiting DFI LP
*/
pub const UDDRC_DRAMTMG15_EN_DFI_LP_T_STAB:u32 = 0x1u32 << 31;

/* -------- UDDRC_ZQCTL0 : (UDDRC_REGS Offset: 0x180)
* ZQ Control Register 0 --------
*/
/* (UDDRC_ZQCTL0) tZQCS for DDR3/DD4/LPDDR2/LPDDR3, tZQLAT for LPDDR4:
* Number of DFI clock cycles of NOP required after a ZQCS
* (ZQ calibration short)/MPC(ZQ Latch) command is issued to SDRAM.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* tZQCS/2 and round it up to the next integer value.
* This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3/LPDDR4
* devices.
*/
pub const UDDRC_ZQCTL0_T_ZQ_SHORT_NOP_POS:u32 = 0;
pub const UDDRC_ZQCTL0_T_ZQ_SHORT_NOP_MSK:u32 = 0x3ffu32 << UDDRC_ZQCTL0_T_ZQ_SHORT_NOP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ZQCTL0_T_ZQ_SHORT_NOP (value:u32) -> u32 {
    UDDRC_ZQCTL0_T_ZQ_SHORT_NOP_MSK & ((value) << UDDRC_ZQCTL0_T_ZQ_SHORT_NOP_POS)
}

/* (UDDRC_ZQCTL0) tZQoper for DDR3/DDR4, tZQCL for LPDDR2/LPDDR3,
* tZQCAL for LPDDR4: Number of DFI clock cycles of NOP required after a
* ZQCL (ZQ calibration long)/MPC(ZQ Start) command is issued to SDRAM.
* When the controller is operating in 1:2 frequency ratio mode:
* DDR3/DDR4: program this to tZQoper/2 and round it up to the next
* integer value.
* LPDDR2/LPDDR3: program this to tZQCL/2 and round it up to the next
* integer value.
* LPDDR4: program this to tZQCAL/2 and round it up to the next integer value.
* This is only present for designs supporting DDR3/DDR4 or
* LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL0_T_ZQ_LONG_NOP_POS:u32 = 16;
pub const UDDRC_ZQCTL0_T_ZQ_LONG_NOP_MSK:u32 = 0x7ffu32 << UDDRC_ZQCTL0_T_ZQ_LONG_NOP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ZQCTL0_T_ZQ_LONG_NOP (value:u32) -> u32 {
    UDDRC_ZQCTL0_T_ZQ_LONG_NOP_MSK & ((value) << UDDRC_ZQCTL0_T_ZQ_LONG_NOP_POS)
}

/* (UDDRC_ZQCTL0)
* - 1 - Denotes that ZQ resistor is shared between ranks.
* Means ZQinit/ZQCL/ZQCS/MPC(ZQ calibration) commands are sent to one rank
* at a time with tZQinit/tZQCL/tZQCS/tZQCAL/tZQLAT timing met between commands
* so that commands to different ranks do not overlap.
* - 0 - ZQ resistor is not shared.
* This is only present for designs supporting DDR3/DDR4 or
* LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL0_ZQ_RESISTOR_SHARED:u32 = 0x1u32 << 29;

/* (UDDRC_ZQCTL0)
* - 1 - Disable issuing of ZQCL/MPC(ZQ calibration) command
* at Self-Refresh/SR-Powerdown exit. Only applicable when run in DDR3 or DDR4
* or LPDDR2 or LPDDR3 or LPDDR4 mode.
* - 0 - Enable issuing of ZQCL/MPC(ZQ calibration) command at
* Self-Refresh/SR-Powerdown exit. Only applicable when run in DDR3 or DDR4
* or LPDDR2 or LPDDR3 or LPDDR4 mode.
* This is only present for designs supporting DDR3/DDR4 or
* LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL0_DIS_SRX_ZQCL:u32 = 0x1u32 << 30;

/* (UDDRC_ZQCTL0)
* - 1 - Disable uMCTL2 generation of ZQCS/MPC(ZQ calibration) command.
* Register DBGCMD.zq_calib_short can be used instead to issue ZQ calibration
* request from APB module.
* - 0 - Internally generate ZQCS/MPC(ZQ calibration) commands based on
* ZQCTL1.t_zq_short_interval_x1024.
* This is only present for designs supporting DDR3/DDR4 or
* LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL0_DIS_AUTO_ZQ:u32 = 0x1u32 << 31;

/* -------- UDDRC_ZQCTL1 : (UDDRC_REGS Offset: 0x184)
* ZQ Control Register 1 --------
*/
/* (UDDRC_ZQCTL1) Average interval to wait between automatically issuing
* ZQCS (ZQ calibration short)/MPC(ZQ calibration) commands to
* DDR3/DDR4/LPDDR2/LPDDR3/LPDDR4 devices.
* Meaningless, if ZQCTL0.dis_auto_zq=1.
* Unit: 1024 DFI clock cycles.
* This is only present for designs supporting DDR3/DDR4 or
* LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024_POS:u32 = 0;
pub const UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024_MSK:u32 = 0xfffffu32 << UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024_POS;


#[allow(non_snake_case)]
pub const fn UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024 (value:u32) -> u32 {
    UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024_MSK & ((value) << UDDRC_ZQCTL1_T_ZQ_SHORT_INTERVAL_X1024_POS)
}

/* (UDDRC_ZQCTL1) tZQReset: Number of DFI clock cycles of NOP required after
* a ZQReset (ZQ calibration Reset) command is issued to SDRAM.
* When the controller is operating in 1:2 frequency ratio mode, program this to
* tZQReset/2 and round it up to the next integer value.
* This is only present for designs supporting LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL1_T_ZQ_RESET_NOP_POS:u32 = 20;
pub const UDDRC_ZQCTL1_T_ZQ_RESET_NOP_MSK:u32 = 0x3ffu32 << UDDRC_ZQCTL1_T_ZQ_RESET_NOP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ZQCTL1_T_ZQ_RESET_NOP (value:u32) -> u32 {
    UDDRC_ZQCTL1_T_ZQ_RESET_NOP_MSK & ((value) << UDDRC_ZQCTL1_T_ZQ_RESET_NOP_POS)
}

/* -------- UDDRC_ZQCTL2 : (UDDRC_REGS Offset: 0x188)
* ZQ Control Register 2 --------
*/
/* (UDDRC_ZQCTL2)
* Setting this register bit to 1 triggers a ZQ Reset operation.
* When the ZQ Reset operation is complete, the uMCTL2 automatically clears
* this bit. It is recommended NOT to set this signal if in Init,
* Self-Refresh(except LPDDR4) or SR-Powerdown(LPDDR4) or Deep power-down
* operating modes.
* This is only present for designs supporting LPDDR2/LPDDR3/LPDDR4 devices.
*/
pub const UDDRC_ZQCTL2_ZQ_RESET:u32 = 0x1u32 << 0;

/* -------- UDDRC_ZQSTAT : (UDDRC_REGS Offset: 0x18C)
* ZQ Status Register --------
*/

/* (UDDRC_ZQSTAT) SoC core may initiate a ZQ Reset operation only if this signal
* is low. This signal goes high in the clock after the uMCTL2 accepts the
* ZQ Reset request. It goes low when the ZQ Reset command is issued to the
* SDRAM and the associated NOP period is over. It is recommended not to
* perform ZQ Reset commands when this signal is high.
* - 0 - Indicates that the SoC core can initiate a ZQ Reset operation
* - 1 - Indicates that ZQ Reset operation is in progress
*/
pub const UDDRC_ZQSTAT_ZQ_RESET_BUSY:u32 = 0x1u32 << 0;
/* -------- UDDRC_DFITMG0 : (UDDRC_REGS Offset: 0x190)
* DFI Timing Register 0 --------
*/

/* (UDDRC_DFITMG0) Write latency
* Number of clocks from the write command to write data enable (dfi_wrdata_en).
* This corresponds to the DFI timing parameter tphy_wrlat.
* Refer to PHY specification for correct value.Note that, depending on the PHY,
* if using RDIMM/LRDIMM, it may be necessary to use the adjusted value of CL
* in the calculation of tphy_wrlat. This is to compensate for the extra
* cycle(s) of latency through the RDIMM/LRDIMM.
* For LPDDR4, dfi_tphy_wrlat>60 is not supported.
* Unit: DFI clock cycles or DFI PHY clock cycles, depending on
* DFITMG0.dfi_wrdata_use_dfi_phy_clk.
*/
pub const UDDRC_DFITMG0_DFI_TPHY_WRLAT_POS:u32 = 0;
pub const UDDRC_DFITMG0_DFI_TPHY_WRLAT_MSK:u32 = 0x3fu32 << UDDRC_DFITMG0_DFI_TPHY_WRLAT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG0_DFI_TPHY_WRLAT (value:u32) -> u32 {
    UDDRC_DFITMG0_DFI_TPHY_WRLAT_MSK & ((value) << UDDRC_DFITMG0_DFI_TPHY_WRLAT_POS)
}

/* (UDDRC_DFITMG0) Specifies the number of clock cycles between when
* dfi_wrdata_en is asserted to when the associated write data is driven
* on the dfi_wrdata signal.  This corresponds to the DFI timing parameter
* tphy_wrdata.  Refer to PHY specification for correct value.
* Note, max supported value is 8.
* Unit: DFI clock cycles or DFI PHY clock cycles, depending on
* DFITMG0.dfi_wrdata_use_dfi_phy_clk.
*/
pub const UDDRC_DFITMG0_DFI_TPHY_WRDATA_POS:u32 = 8;
pub const UDDRC_DFITMG0_DFI_TPHY_WRDATA_MSK:u32 = 0x3fu32 << UDDRC_DFITMG0_DFI_TPHY_WRDATA_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG0_DFI_TPHY_WRDATA (value:u32) -> u32 {
    UDDRC_DFITMG0_DFI_TPHY_WRDATA_MSK & ((value) << UDDRC_DFITMG0_DFI_TPHY_WRDATA_POS)
}

/* (UDDRC_DFITMG0) Defines whether dfi_wrdata_en/dfi_wrdata/dfi_wrdata_mask is
* generated using HDR (DFI clock) or SDR (DFI PHY clock) values
* Selects whether value in DFITMG0.dfi_tphy_wrlat  is in terms of
* HDR (DFI clock) or SDR (DFI PHY clock) cycles
* Selects whether value in DFITMG0.dfi_tphy_wrdata is in terms of
* HDR (DFI clock) or SDR (DFI PHY clock) cycles
* - 0 in terms of HDR (DFI clock) cycles
* - 1 in terms of SDR (DFI PHY clock) cycles
* Refer to PHY specification for correct value.
* If using a Synopsys DWC DDR3/2 PHY, DWC DDR2/3-Lite/mDDR PHY,
* DWC DDR multiPHY or DWC Gen2 DDR multiPHY, this field must be set to 0;
* otherwise:
* - If MEMC_PROG_FREQ_RATIO=1 and MSTR.frequency_ratio=1,
this field should be set to 0
* - Else, it must be set to 1
*/
pub const UDDRC_DFITMG0_DFI_WRDATA_USE_DFI_PHY_CLK:u32 = 0x1u32 << 15;

/* (UDDRC_DFITMG0) Time from the assertion of a read command on the
* DFI interface to the assertion of the dfi_rddata_en signal.
* Refer to PHY specification for correct value.
* This corresponds to the DFI parameter trddata_en.
* Note that, depending on the PHY, if using RDIMM/LRDIMM, it may be necessary
* to use the adjusted value of CL in the calculation of trddata_en.
* This is to compensate for the extra cycle(s) of latency through the
* RDIMM/LRDIMM.
* Unit: DFI clock cycles or DFI PHY clock cycles, depending on
* DFITMG0.dfi_rddata_use_dfi_phy_clk.
*/
pub const UDDRC_DFITMG0_DFI_T_RDDATA_EN_POS:u32 = 16;
pub const UDDRC_DFITMG0_DFI_T_RDDATA_EN_MSK:u32 = 0x7fu32 << UDDRC_DFITMG0_DFI_T_RDDATA_EN_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG0_DFI_T_RDDATA_EN (value:u32) -> u32 {
    UDDRC_DFITMG0_DFI_T_RDDATA_EN_MSK & ((value) << UDDRC_DFITMG0_DFI_T_RDDATA_EN_POS)
}

/* (UDDRC_DFITMG0) Defines whether dfi_rddata_en/dfi_rddata/dfi_rddata_valid is
* generated using HDR (DFI clock) or SDR (DFI PHY clock) values.
* Selects whether value in DFITMG0.dfi_t_rddata_en is in terms of
* HDR (DFI clock) or SDR (DFI PHY clock) cycles:
* - 0 in terms of HDR (DFI clock) cycles
* - 1 in terms of SDR (DFI PHY clock) cycles
* Refer to PHY specification for correct value.
* If using a Synopsys DWC DDR3/2 PHY, DWC DDR2/3-Lite/mDDR PHY,
* DWC DDR multiPHY or DWC Gen2 DDR multiPHY, this field must be set to 0;
* otherwise:
* - If MEMC_PROG_FREQ_RATIO=1 and MSTR.frequency_ratio=1, this field should be
set to 0
* - Else, it must be set to 1
*/
pub const UDDRC_DFITMG0_DFI_RDDATA_USE_DFI_PHY_CLK:u32 = 0x1u32 << 23;

/* (UDDRC_DFITMG0) Specifies the number of DFI clock cycles after an assertion
* or de-assertion of the DFI control signals that the control signals at the
* PHY-DRAM interface reflect the assertion or de-assertion.
* If the DFI clock and the memory clock are not phase-aligned, this timing
* parameter should be rounded up to the next integer value.
* Note that if using RDIMM/LRDIMM, it is necessary to increment this parameter
* by RDIMM's/LRDIMM's extra cycle of latency in terms of DFI clock.
*/
pub const UDDRC_DFITMG0_DFI_T_CTRL_DELAY_POS:u32 = 24;
pub const UDDRC_DFITMG0_DFI_T_CTRL_DELAY_MSK:u32 = 0x1fu32 << UDDRC_DFITMG0_DFI_T_CTRL_DELAY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG0_DFI_T_CTRL_DELAY (value:u32) -> u32 {
    UDDRC_DFITMG0_DFI_T_CTRL_DELAY_MSK & ((value) << UDDRC_DFITMG0_DFI_T_CTRL_DELAY_POS)
}

/* -------- UDDRC_DFITMG1 : (UDDRC_REGS Offset: 0x194)
* DFI Timing Register 1 --------
*/
/* (UDDRC_DFITMG1) Specifies the number of DFI clock cycles from the
* de-assertion of the dfi_dram_clk_disable signal on the DFI until the first
* valid rising edge of the clock to the DRAM memory devices,
* at the PHY-DRAM boundary. If the DFI clock and the memory clock are not
* phase aligned, this timing parameter should be rounded up to the next
* integer value.
*/
pub const UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE_POS:u32 = 0;
pub const UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE_MSK:u32 = 0x1fu32 << UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE (value:u32) -> u32 {
    UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE_MSK & ((value) << UDDRC_DFITMG1_DFI_T_DRAM_CLK_ENABLE_POS)
}

/* (UDDRC_DFITMG1) Specifies the number of DFI clock cycles from the assertion
* of the dfi_dram_clk_disable signal on the DFI until the clock to the DRAM
* memory devices, at the PHY-DRAM boundary, maintains a low value.
* If the DFI clock and the memory clock are not phase aligned, this timing
* parameter should be rounded up to the next integer value.
*/
pub const UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE_POS:u32 = 8;
pub const UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE_MSK:u32 = 0x1fu32 << UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE (value:u32) -> u32 {
    UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE_MSK & ((value) << UDDRC_DFITMG1_DFI_T_DRAM_CLK_DISABLE_POS)
}

/* (UDDRC_DFITMG1) Specifies the number of DFI clock cycles between when the
* dfi_wrdata_en signal is asserted and when the corresponding write data
* transfer is completed on the DRAM bus.
* This corresponds to the DFI timing parameter twrdata_delay.
* Refer to PHY specification for correct value.
* For DFI 3.0 PHY, set to twrdata_delay, a new timing parameter introduced
* in DFI 3.0.
* For DFI 2.1 PHY, set to tphy_wrdata + (delay of DFI write data to the DRAM).
* Value to be programmed is in terms of DFI clocks, not PHY clocks.
* In FREQ_RATIO=2, divide PHY's value by 2 and round up to next integer.
* If using DFITMG0.dfi_wrdata_use_dfi_phy_clk=1, add 1 to the value.
* Unit: Clocks
*/
pub const UDDRC_DFITMG1_DFI_T_WRDATA_DELAY_POS:u32 = 16;
pub const UDDRC_DFITMG1_DFI_T_WRDATA_DELAY_MSK:u32 = 0x1fu32 << UDDRC_DFITMG1_DFI_T_WRDATA_DELAY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG1_DFI_T_WRDATA_DELAY (value:u32) -> u32 {
    UDDRC_DFITMG1_DFI_T_WRDATA_DELAY_MSK & ((value) << UDDRC_DFITMG1_DFI_T_WRDATA_DELAY_POS)
}

/* (UDDRC_DFITMG1) */
pub const UDDRC_DFITMG1_DFI_T_PARRIN_LAT_POS:u32 = 24;
pub const UDDRC_DFITMG1_DFI_T_PARRIN_LAT_MSK:u32 = 0x3u32 << UDDRC_DFITMG1_DFI_T_PARRIN_LAT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG1_DFI_T_PARRIN_LAT (value:u32) -> u32 {
    UDDRC_DFITMG1_DFI_T_PARRIN_LAT_MSK & ((value) << UDDRC_DFITMG1_DFI_T_PARRIN_LAT_POS)
}

/* (UDDRC_DFITMG1) */
pub const UDDRC_DFITMG1_DFI_T_CMD_LAT_POS:u32 = 28;
pub const UDDRC_DFITMG1_DFI_T_CMD_LAT_MSK:u32 = 0xfu32 << UDDRC_DFITMG1_DFI_T_CMD_LAT_POS;


#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG1_DFI_T_CMD_LAT (value:u32) -> u32 {
    UDDRC_DFITMG1_DFI_T_CMD_LAT_MSK & ((value) << UDDRC_DFITMG1_DFI_T_CMD_LAT_POS)
}

/* -------- UDDRC_DFILPCFG0 : (UDDRC_REGS Offset: 0x198)
* DFI Low Power Configuration Register 0 --------
*/
/* (UDDRC_DFILPCFG0) Enables DFI Low Power interface handshaking
* during Power Down Entry/Exit.
* - 0 - Disabled
* - 1 - Enabled
*/
pub const UDDRC_DFILPCFG0_DFI_LP_EN_PD:u32 = 0x1u32 << 0;

/* (UDDRC_DFILPCFG0) Value in DFI clock cycles to drive on dfi_lp_wakeup signal
* when Power Down mode is entered.
* Determines the DFI's tlp_wakeup time:
- 0x0 - 16 cycles
- 0x1 - 32 cycles
- 0x2 - 64 cycles
- 0x3 - 128 cycles
- 0x4 - 256 cycles
- 0x5 - 512 cycles
- 0x6 - 1024 cycles
- 0x7 - 2048 cycles
- 0x8 - 4096 cycles
- 0x9 - 8192 cycles
- 0xA - 16384 cycles
- 0xB - 32768 cycles
- 0xC - 65536 cycles
- 0xD - 131072 cycles
- 0xE - 262144 cycles
- 0xF - Unlimited
*/
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD_POS:u32 = 4;
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD_MSK:u32 = 0xfu32 << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD (value:u32) -> u32 {
    UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD_MSK & ((value) << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_PD_POS)
}

/* (UDDRC_DFILPCFG0) Enables DFI Low Power interface handshaking during
* Self Refresh Entry/Exit.
* - 0 - Disabled
*  - 1 - Enabled
*/

pub const UDDRC_DFILPCFG0_DFI_LP_EN_SR:u32 = 0x1u32 << 8;
/* (UDDRC_DFILPCFG0) Value in DFI clpck cycles to drive on dfi_lp_wakeup signal
* when Self Refresh mode is entered.
* Determines the DFI's tlp_wakeup time:
- 0x0 - 16 cycles
- 0x1 - 32 cycles
- 0x2 - 64 cycles
- 0x3 - 128 cycles
- 0x4 - 256 cycles
- 0x5 - 512 cycles
- 0x6 - 1024 cycles
- 0x7 - 2048 cycles
- 0x8 - 4096 cycles
- 0x9 - 8192 cycles
- 0xA - 16384 cycles
- 0xB - 32768 cycles
- 0xC - 65536 cycles
- 0xD - 131072 cycles
- 0xE - 262144 cycles
- 0xF - Unlimited
*/
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR_POS:u32 = 12;
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR_MSK:u32 = 0xfu32 << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR (value:u32) -> u32 {
    UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR_MSK & ((value) << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_SR_POS)
}

/* (UDDRC_DFILPCFG0) Enables DFI Low Power interface handshaking during
* Deep Power Down Entry/Exit.
- 0 - Disabled
- 1 - Enabled
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3 devices.
*/
pub const UDDRC_DFILPCFG0_DFI_LP_EN_DPD:u32 = 0x1u32 << 16;

/* (UDDRC_DFILPCFG0) Value in DFI clock cycles to drive on dfi_lp_wakeup signal
* when Deep Power Down mode is entered.
* Determines the DFI's tlp_wakeup time:
- 0x0 - 16 cycles
- 0x1 - 32 cycles
- 0x2 - 64 cycles
- 0x3 - 128 cycles
- 0x4 - 256 cycles
- 0x5 - 512 cycles
- 0x6 - 1024 cycles
- 0x7 - 2048 cycles
- 0x8 - 4096 cycles
- 0x9 - 8192 cycles
- 0xA - 16384 cycles
- 0xB - 32768 cycles
- 0xC - 65536 cycles
- 0xD - 131072 cycles
- 0xE - 262144 cycles
- 0xF - Unlimited
* This is only present for designs supporting mDDR or LPDDR2/LPDDR3 devices.
*/
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD_POS:u32 = 20;
pub const UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD_MSK:u32 = 0xfu32 << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD (value:u32) -> u32 {
    UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD_MSK & ((value) << UDDRC_DFILPCFG0_DFI_LP_WAKEUP_DPD_POS)
}

/* (UDDRC_DFILPCFG0) Setting in DFI clock cycles for DFI's tlp_resp time.
* Same value is used for both Power Down, Self Refresh, Deep Power Down and
* Maximum Power Saving modes.
* DFI 2.1 specification onwards, recommends using a fixed value of 7 always.
*/
pub const UDDRC_DFILPCFG0_DFI_TLP_RESP_POS:u32 = 24;
pub const UDDRC_DFILPCFG0_DFI_TLP_RESP_MSK:u32 = 0x1fu32 << UDDRC_DFILPCFG0_DFI_TLP_RESP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFILPCFG0_DFI_TLP_RESP (value:u32) -> u32 {
    UDDRC_DFILPCFG0_DFI_TLP_RESP_MSK & ((value) << UDDRC_DFILPCFG0_DFI_TLP_RESP_POS)
}

/* -------- UDDRC_DFILPCFG1 : (UDDRC_REGS Offset: 0x19C)
* DFI Low Power Configuration Register 1 --------
*/
pub const UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM_POS:u32 = 4;
pub const UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM_MSK:u32 = 0xfu32 << UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM (value:u32) -> u32 {
    UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM_MSK & ((value) << UDDRC_DFILPCFG1_DFI_LP_WAKEUP_MPSM_POS)
}

pub const UDDRC_DFILPCFG1_DFI_LP_EN_MPSM:u32 = 0x1u32 << 0;
/* -------- UDDRC_DFIUPD0 : (UDDRC_REGS Offset: 0x1A0)
* DFI Update Register 0 --------
*/
/* (UDDRC_DFIUPD0) Specifies the minimum number of DFI clock cycles that the
* dfi_ctrlupd_req signal must be asserted. The uMCTL2 expects the PHY to
* respond within this time.  If the PHY does not respond, the uMCTL2 will
* de-assert dfi_ctrlupd_req after dfi_t_ctrlup_min + 2 cycles.
* Lowest value to assign to this variable is 0x3.
*/
pub const UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN_POS:u32 = 0;
pub const UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN_MSK:u32 = 0x3ffu32 << UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN (value:u32) -> u32 {
    UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN_MSK & ((value) << UDDRC_DFIUPD0_DFI_T_CTRLUP_MIN_POS)
}

/* (UDDRC_DFIUPD0) Specifies the maximum number of DFI clock cycles that the
* dfi_ctrlupd_req signal can assert.
* Lowest value to assign to this variable is 0x40.
*/
pub const UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX_POS:u32 = 16;
pub const UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX_MSK:u32 = 0x3ffu32 << UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX (value:u32) -> u32 {
    UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX_MSK & ((value) << UDDRC_DFIUPD0_DFI_T_CTRLUP_MAX_POS)
}

/* (UDDRC_DFIUPD0) Selects dfi_ctrlupd_req requirements at SRX:
*  - 0 : send ctrlupd after SRX
* - 1 : send ctrlupd before SRX
* If DFIUPD0.dis_auto_ctrlupd_srx=1, this register has no impact,
* because no dfi_ctrlupd_req will be issued when SRX.
*/
pub const UDDRC_DFIUPD0_CTRLUPD_PRE_SRX:u32 = 0x1u32 << 29;

/* (UDDRC_DFIUPD0) When '1', disable the automatic dfi_ctrlupd_req generation
* by the uMCTL2 at self-refresh exit.
* When '0', uMCTL2 issues a dfi_ctrlupd_req before or after exiting
* self-refresh,  depending on DFIUPD0.ctrlupd_pre_srx.
*/
pub const UDDRC_DFIUPD0_DIS_AUTO_CTRLUPD_SRX:u32 = 0x1u32 << 30;

/* (UDDRC_DFIUPD0) When '1', disable the automatic dfi_ctrlupd_req generation
* by the uMCTL2. The core must issue the dfi_ctrlupd_req signal using register
* DBGCMD.ctrlupd.
* When '0', uMCTL2 issues dfi_ctrlupd_req periodically.
*/
pub const UDDRC_DFIUPD0_DIS_AUTO_CTRLUPD:u32 = 0x1u32 << 31;

/* -------- UDDRC_DFIUPD1 : (UDDRC_REGS Offset: 0x1A4)
* DFI Update Register 1 --------
*/
/* (UDDRC_DFIUPD1) This is the maximum amount of time between uMCTL2 initiated
* DFI update requests. This timer resets with each update request;
* when the timer expires dfi_ctrlupd_req is sent and traffic is blocked
* until the dfi_ctrlupd_ackx is received. PHY can use this idle time to
* recalibrate the delay lines to the DLLs. The DFI controller update is also
* used to reset PHY FIFO pointers in case of data capture errors.
* Updates are required to maintain calibration over PVT, but frequent updates
* may impact performance. Minimum allowed value for this field is 1.
* Note: Value programmed for DFIUPD1.dfi_t_ctrlupd_interval_max_x1024 must be
* greater than DFIUPD1.dfi_t_ctrlupd_interval_min_x1024.
* Unit: 1024 DFI clock cycles
*/
pub const UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024_POS:u32 = 0;
pub const UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024_MSK:u32 = 0xffu32 << UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024 (value:u32) -> u32 {
    UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024_MSK & ((value) << UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MAX_X1024_POS)
}

/* (UDDRC_DFIUPD1) This is the minimum amount of time between uMCTL2 initiated
* DFI update requests (which is executed whenever the uMCTL2 is idle).
* Set this number higher to reduce the frequency of update requests,
* which can have a small impact on the latency of the first read request
* when the uMCTL2 is idle. Minimum allowed value for this field is 1.
* Unit: 1024 DFI clock cycles
*/
pub const UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024_POS:u32 = 16;
pub const UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024_MSK:u32 = 0xffu32 << UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024 (value:u32) -> u32 {
    UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024_MSK & ((value) << UDDRC_DFIUPD1_DFI_T_CTRLUPD_INTERVAL_MIN_X1024_POS)
}

/* -------- UDDRC_DFIUPD2 : (UDDRC_REGS Offset: 0x1A8)
* DFI Update Register 2 --------
*/
/* (UDDRC_DFIUPD2) Enables the support for acknowledging PHY-initiated updates:
*    - 0 - Disabled
*   - 1 - Enabled
*/
pub const UDDRC_DFIUPD2_DFI_PHYUPD_EN:u32 = 0x1u32 << 31;
/* -------- UDDRC_DFIMISC : (UDDRC_REGS Offset: 0x1B0)
* DFI Miscellaneous Control Register --------
*/
/* (UDDRC_DFIMISC) PHY initialization complete enable signal.
* When asserted the dfi_init_complete signal can be used to trigger
* SDRAM initialisation
*/
pub const UDDRC_DFIMISC_DFI_INIT_COMPLETE_EN:u32 = 0x1u32 << 0;

/* (UDDRC_DFIMISC) Enables support of ctl_idle signal, which is non-DFI related
* pin specific to certain Synopsys PHYs. See signal description of ctl_idle
* signal for further details of ctl_idle functionality.
*/
pub const UDDRC_DFIMISC_CTL_IDLE_EN:u32 = 0x1u32 << 4;

/* (UDDRC_DFIMISC) PHY init start request signal.When asserted it triggers
* the PHY init start request
*/
pub const UDDRC_DFIMISC_DFI_INIT_START:u32 = 0x1u32 << 5;

pub const UDDRC_DFIMISC_DIS_DYN_ADR_TRI:u32 = 0x1u32 << 6;

pub const UDDRC_DFIMISC_LP_OPTIMIZED_WRITE:u32 = 0x1u32 << 7;

/* (UDDRC_DFIMISC) Indicates the operating frequency of the system.
* The number of supported frequencies and the mapping of signal values to clock
* frequencies are defined by the PHY.
*/
pub const UDDRC_DFIMISC_DFI_FREQUENCY_POS:u32 = 8;
pub const UDDRC_DFIMISC_DFI_FREQUENCY_MSK:u32 = 0x1fu32 << UDDRC_DFIMISC_DFI_FREQUENCY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFIMISC_DFI_FREQUENCY (value:u32) -> u32 {
    UDDRC_DFIMISC_DFI_FREQUENCY_MSK & ((value) << UDDRC_DFIMISC_DFI_FREQUENCY_POS)
}

/* -------- UDDRC_DFITMG2 : (UDDRC_REGS Offset: 0x1B4)
* DFI Timing Register 2 --------
*/
/* (UDDRC_DFITMG2) Number of DFI PHY clock cycles between when a write command
* is sent on the DFI control interface and when the associated dfi_wrdata_cs
* signal is asserted.
* This corresponds to the DFI timing parameter tphy_wrcslat.
*/
pub const UDDRC_DFITMG2_DFI_TPHY_WRCSLAT_POS:u32 = 0;
pub const UDDRC_DFITMG2_DFI_TPHY_WRCSLAT_MSK:u32 = 0x1fu32 << UDDRC_DFITMG2_DFI_TPHY_WRCSLAT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG2_DFI_TPHY_WRCSLAT (value:u32) -> u32 {
    UDDRC_DFITMG2_DFI_TPHY_WRCSLAT_MSK & ((value) << UDDRC_DFITMG2_DFI_TPHY_WRCSLAT_POS)
}

/* (UDDRC_DFITMG2) Number of DFI PHY clock cycles between when a read command
* is sent on the DFI control interface and when the associated
* dfi_rddata_cs signal is asserted.
* This corresponds to the DFI timing parameter tphy_rdcslat.
* Refer to PHY specification for correct value.
*/
pub const UDDRC_DFITMG2_DFI_TPHY_RDCSLAT_POS:u32 = 8;
pub const UDDRC_DFITMG2_DFI_TPHY_RDCSLAT_MSK:u32 = 0x7fu32 << UDDRC_DFITMG2_DFI_TPHY_RDCSLAT_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG2_DFI_TPHY_RDCSLAT (value:u32) -> u32 {
    UDDRC_DFITMG2_DFI_TPHY_RDCSLAT_MSK & ((value) << UDDRC_DFITMG2_DFI_TPHY_RDCSLAT_POS)
}

pub const UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY_POS:u32 = 0;
pub const UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY_MSK:u32 = 0x1fu32 << UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY (value:u32) -> u32 {
    UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY_MSK & ((value) << UDDRC_DFITMG3_DFI_T_GEARDOWN_DELAY_POS)
}

/* -------- UDDRC_DFISTAT : (UDDRC_REGS Offset: 0x1BC)
* DFI Status Register --------
*/
/* (UDDRC_DFISTAT) The status flag register which announces when the DFI
* initialization has been completed. The DFI INIT triggered by dfi_init_start
* signal and then the dfi_init_complete flag is polled to know when the
* initialization is done.
*/
pub const UDDRC_DFISTAT_DFI_INIT_COMPLETE:u32 = 0x1u32 << 0;

/* (UDDRC_DFISTAT) Stores the value of the dfi_lp_ack input to the controller.
*/
pub const UDDRC_DFISTAT_DFI_LP_ACK:u32 = 0x1u32 << 1;

pub const UDDRC_DBICTL_DM_EN:u32 = 0x1u32 << 0;

pub const UDDRC_DBICTL_WR_DBI_EN:u32 = 0x1u32 << 1;

pub const UDDRC_DBICTL_RD_DBI_EN:u32 = 0x1u32 << 2;

/* -------- UDDRC_DFIPHYMSTR : (UDDRC_REGS Offset: 0x1C4)
* DFI PHY Master --------
*/
/* (UDDRC_DFIPHYMSTR) Enables the PHY Master Interface:
*   - 0 - Disabled
*   - 1 - Enabled
*/
pub const UDDRC_DFIPHYMSTR_DFI_PHYMSTR_EN:u32 = 0x1u32 << 0;

/* -------- UDDRC_ADDRMAP1 : (UDDRC_REGS Offset: 0x204)
* Address Map Register 1 --------
*/
/* (UDDRC_ADDRMAP1) Selects the HIF address bits used as bank address bit 0.
* Valid Range: 0 to 32 and 63
* Internal Base: 2
* The selected HIF address bit for each of the bank address bits is determined
* by adding the internal base to the value of this field.
* If unused, set to 63 and then bank address bit 0 is set to 0.
*/
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B0_POS:u32 = 0;
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B0_MSK:u32 = 0x3fu32 << UDDRC_ADDRMAP1_ADDRMAP_BANK_B0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP1_ADDRMAP_BANK_B0 (value:u32) -> u32 {
    UDDRC_ADDRMAP1_ADDRMAP_BANK_B0_MSK & ((value) << UDDRC_ADDRMAP1_ADDRMAP_BANK_B0_POS)
}

/* (UDDRC_ADDRMAP1) Selects the HIF address bits used as bank address bit 1.
* Valid Range: 0 to 32 and 63
* Internal Base: 3
* The selected HIF address bit for each of the bank address bits is determined
* by adding the internal base to the value of this field.
* If unused, set to 63 and then bank address bit 1 is set to 0.
*/
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B1_POS:u32 = 8;
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B1_MSK:u32 = 0x3fu32 << UDDRC_ADDRMAP1_ADDRMAP_BANK_B1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP1_ADDRMAP_BANK_B1 (value:u32) -> u32 {
    UDDRC_ADDRMAP1_ADDRMAP_BANK_B1_MSK & ((value) << UDDRC_ADDRMAP1_ADDRMAP_BANK_B1_POS)
}

/* (UDDRC_ADDRMAP1) Selects the HIF address bit used as bank address bit 2.
* Valid Range: 0 to 31 and 63
* Internal Base: 4
* The selected HIF address bit is determined by adding the internal base to
* the value of this field.
* If unused, set to 63 and then bank address bit 2 is set to 0.
*/
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B2_POS:u32 = 16;
pub const UDDRC_ADDRMAP1_ADDRMAP_BANK_B2_MSK:u32 = 0x3fu32 << UDDRC_ADDRMAP1_ADDRMAP_BANK_B2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP1_ADDRMAP_BANK_B2 (value:u32) -> u32 {
    UDDRC_ADDRMAP1_ADDRMAP_BANK_B2_MSK & ((value) << UDDRC_ADDRMAP1_ADDRMAP_BANK_B2_POS)
}

/* -------- UDDRC_ADDRMAP2 : (UDDRC_REGS Offset: 0x208)
* Address Map Register 2 --------
*/
/* (UDDRC_ADDRMAP2)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 2.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 3.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 4.
* Valid Range: 0 to 7
* Internal Base: 2
* The selected HIF address bit is determined by adding the internal base to
* the value of this field.
* Note, if UMCTL2_INCL_ARB=1 and MEMC_BURST_LENGTH=8, it is required to program
* this to 0 unless:
- in Half or Quarter bus width (MSTR.data_bus_width!=00) and
- PCCFG.bl_exp_mode==1 and either
- In DDR4   and ADDRMAP8.addrmap_bg_b0==0 or
- In LPDDR4 and ADDRMAP1.addrmap_bank_b0==0
* If UMCTL2_INCL_ARB=1 and MEMC_BURST_LENGTH=16, it is required to program this
* to 0 unless:
- in Half or Quarter bus width (MSTR.data_bus_width!=00) and
- PCCFG.bl_exp_mode==1 and
- In DDR4 and ADDRMAP8.addrmap_bg_b0==0
* Otherwise, if MEMC_BURST_LENGTH=8 and Full Bus Width
* (MSTR.data_bus_width==00), it is recommended to program this to 0 so that
* HIF[2] maps to column address bit 2.
*
* If MEMC_BURST_LENGTH=16 and Full Bus Width (MSTR.data_bus_width==00), it is
* recommended to program this to 0 so that HIF[2] maps to column address bit 2.
*
* If MEMC_BURST_LENGTH=16 and Half Bus Width (MSTR.data_bus_width==01), it is
* recommended to program this to 0 so that HIF[2] maps to column address bit 3.
*/
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B2_POS:u32 = 0;
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B2_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP2_ADDRMAP_COL_B2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP2_ADDRMAP_COL_B2 (value:u32) -> u32 {
    UDDRC_ADDRMAP2_ADDRMAP_COL_B2_MSK & ((value) << UDDRC_ADDRMAP2_ADDRMAP_COL_B2_POS)
}


pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B3_POS:u32 = 8;
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B3_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP2_ADDRMAP_COL_B3_POS;
/* (UDDRC_ADDRMAP2)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 3.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 4.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 5.
* Valid Range: 0 to 7
* Internal Base: 3
* The selected HIF address bit is determined by adding the internal base to
* the value of this field.
* Note, if UMCTL2_INCL_ARB=1, MEMC_BURST_LENGTH=16, Full bus width
* (MSTR.data_bus_width=00) and BL16 (MSTR.burst_rdwr=1000),
* it is recommended to program this to 0.
*/
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP2_ADDRMAP_COL_B3 (value:u32) -> u32 {
    UDDRC_ADDRMAP2_ADDRMAP_COL_B3_MSK & ((value) << UDDRC_ADDRMAP2_ADDRMAP_COL_B3_POS)
}

/**< \brief (UDDRC_ADDRMAP2)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 4.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 5.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 6.
* Valid Range: 0 to 7, and 15
* Internal Base: 4
* The selected HIF address bit is determined by adding the internal base to the
* value of this field. If unused, set to 15 and then this column address bit is
* set to 0.
*/
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B4_POS:u32 = 16;
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B4_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP2_ADDRMAP_COL_B4_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP2_ADDRMAP_COL_B4 (value:u32) -> u32 {
    UDDRC_ADDRMAP2_ADDRMAP_COL_B4_MSK & ((value) << UDDRC_ADDRMAP2_ADDRMAP_COL_B4_POS)
}

/* (UDDRC_ADDRMAP2)
* - Full bus width mode: Selects the HIF address bit used as column address
bit 5.
* - Half bus width mode: Selects the HIF address bit used as column address
bit 6.
* - Quarter bus width mode: Selects the HIF address bit used as column address
bit 7 .
* Valid Range: 0 to 7, and 15
* Internal Base: 5
* The selected HIF address bit is determined by adding the internal base to the
* value of this field. If unused, set to 15 and then this column address bit is
* set to 0.
*/
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B5_POS:u32 = 24;
pub const UDDRC_ADDRMAP2_ADDRMAP_COL_B5_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP2_ADDRMAP_COL_B5_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP2_ADDRMAP_COL_B5 (value:u32) -> u32 {
    UDDRC_ADDRMAP2_ADDRMAP_COL_B5_MSK & ((value) << UDDRC_ADDRMAP2_ADDRMAP_COL_B5_POS)
}

/* -------- UDDRC_ADDRMAP3 : (UDDRC_REGS Offset: 0x20C)
* Address Map Register 3 --------
*/
/* (UDDRC_ADDRMAP3)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 6.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 7.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 8.
* Valid Range: 0 to 7, and 15
* Internal Base: 6
* The selected HIF address bit is determined by adding the internal base to the
* value of this field. If unused, set to 15 and then this column address bit is
* set to 0.
*/
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B6_POS:u32 = 0;
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B6_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP3_ADDRMAP_COL_B6_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP3_ADDRMAP_COL_B6 (value:u32) -> u32 {
    UDDRC_ADDRMAP3_ADDRMAP_COL_B6_MSK & ((value) << UDDRC_ADDRMAP3_ADDRMAP_COL_B6_POS)
}

/* (UDDRC_ADDRMAP3)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 7.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 8.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 9.
* Valid Range: 0 to 7, x, and 31. x indicate a valid value in inline
* ECC configuration.
* Internal Base: 7
* The selected HIF address bit is determined by adding the internal base to the
* value of this field. If unused, set to 31 and then this column address bit is
* set to 0.
*
* In Inline ECC configuration (MEMC_INLINE_ECC=1) and ECC is enabled
* (ECCCFG0.ecc_mode>0), the highest 3 column address bits must map to the
* highest 3 valid HIF address bits.
*
* If column bit 7 is the third highest column address bit, it must map to the
* third highest valid HIF address bit.
* (x = the highest valid HIF address bit - 2 - internal base)
*
* if it is unused, set to 31.
*/
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B7_POS:u32 = 8;
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B7_MSK:u32 = 0x1fu32 << UDDRC_ADDRMAP3_ADDRMAP_COL_B7_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP3_ADDRMAP_COL_B7 (value:u32) -> u32 {
    UDDRC_ADDRMAP3_ADDRMAP_COL_B7_MSK & ((value) << UDDRC_ADDRMAP3_ADDRMAP_COL_B7_POS)
}

/* (UDDRC_ADDRMAP3)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 8.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 9.
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 11 (10 in LPDDR2/LPDDR3 mode).
* Valid Range: 0 to 7, x, and 31. x indicate a valid value in inline ECC
* configuration.
* Internal Base: 8
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 31 and then this column address bit is set to 0.
*
* Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved
* for indicating auto-precharge, and hence no source address bit can be mapped
* to column address bit 10.
*
* In LPDDR2/LPDDR3, there is a dedicated bit for auto-precharge in the CA bus
* and hence column bit 10 is used.
*
* In Inline ECC configuration (MEMC_INLINE_ECC=1) and ECC is enabled
* (ECCCFG0.ecc_mode>0), the highest 3 column address bits must map to the
* highest 3 valid HIF address bits.
*
* If column bit 8 is the second highest column address bit, it must map to the
* second highest valid HIF address bit.
* (x = the highest valid HIF address bit - 1 - internal base)
*
* If column bit 8 is the third highest column address bit, it must map to the
* third highest valid HIF address bit.
* (x = the highest valid HIF address bit - 2 - internal base)
*
* if it is unused, set to 31.
*/
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B8_POS:u32 = 16;
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B8_MSK:u32 = 0x1fu32 << UDDRC_ADDRMAP3_ADDRMAP_COL_B8_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP3_ADDRMAP_COL_B8 (value:u32) -> u32 {
    UDDRC_ADDRMAP3_ADDRMAP_COL_B8_MSK & ((value) << UDDRC_ADDRMAP3_ADDRMAP_COL_B8_POS)
}
/* (UDDRC_ADDRMAP3)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 9.
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 11 (10 in LPDDR2/LPDDR3 mode).
*  - Quarter bus width mode: Selects the HIF address bit used as column address
bit 13 (11 in LPDDR2/LPDDR3 mode).
* Valid Range: 0 to 7, x, and 31. x indicate a valid value in inline ECC
* configuration.
* Internal Base: 9
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 31 and then this column address bit is set to 0.
*
* Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved
* for indicating auto-precharge, and hence no source address bit can be mapped
* to column address bit 10.
*
* In LPDDR2/LPDDR3, there is a dedicated bit for auto-precharge in the CA bus
* and hence column bit 10 is used.
*
* In Inline ECC configuration (MEMC_INLINE_ECC=1) and ECC is enabled
* (ECCCFG0.ecc_mode>0), the highest 3 column address bits must map to the
* highest 3 valid HIF address bits.
*
* If column bit 9 is the highest column address bit, it must map to the highest
* valid HIF address bit. (x = the highest valid HIF address bit - internal base)
* If column bit 9 is the second highest column address bit, it must map to the
* second highest valid HIF address bit.
* (x = the highest valid HIF address bit - 1 - internal base)
*
* If column bit 9 is the third highest column address bit, it must map to the
* third highest valid HIF address bit.
* (x = the highest valid HIF address bit - 2 - internal base)
*
* if it is unused, set to 31.
*/
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B9_POS:u32 = 24;
pub const UDDRC_ADDRMAP3_ADDRMAP_COL_B9_MSK:u32 = 0x1fu32 << UDDRC_ADDRMAP3_ADDRMAP_COL_B9_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP3_ADDRMAP_COL_B9 (value:u32) -> u32 {
    UDDRC_ADDRMAP3_ADDRMAP_COL_B9_MSK & ((value) << UDDRC_ADDRMAP3_ADDRMAP_COL_B9_POS)
}

/* -------- UDDRC_ADDRMAP4 : (UDDRC_REGS Offset: 0x210)
* Address Map Register 4 --------
*/
/* (UDDRC_ADDRMAP4)
*  - Full bus width mode: Selects the HIF address bit used as column address
bit 11 (10 in LPDDR2/LPDDR3 mode).
*  - Half bus width mode: Selects the HIF address bit used as column address
bit 13 (11 in LPDDR2/LPDDR3 mode).
*  - Quarter bus width mode: UNUSED. To make it unused, this must be tied to
4'hF.
* Valid Range: 0 to 7, x, and 31. x indicate a valid value in inline ECC
configuration.
* Internal Base: 10
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 31 and then this column address bit is set to 0.
*
* Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved
* for indicating auto-precharge, and hence no source address bit can be mapped
* to column address bit 10.
* In LPDDR2/LPDDR3, there is a dedicated bit for auto-precharge in the CA bus
* and hence column bit 10 is used.
*
* In Inline ECC configuration (MEMC_INLINE_ECC=1) and ECC is enabled
* (ECCCFG0.ecc_mode>0), the highest 3 column address bits must map to the
* highest 3 valid HIF address bits.
*
* If column bit 10 is the highest column address bit, it must map to the
* highest valid HIF address bit.
* (x = the highest valid HIF address bit - internal base)
*
* If column bit 10 is the second highest column address bit, it must map to the
* second highest valid HIF address bit.
* (x = the highest valid HIF address bit - 1 - internal base)
*
* If column bit 10 is the third highest column address bit, it must map to the
* third highest valid HIF address bit.
* (x = the highest valid HIF address bit - 2 - internal base)
*
* if it is unused, set to 31.
*/
pub const UDDRC_ADDRMAP4_ADDRMAP_COL_B10_POS:u32 = 0;
pub const UDDRC_ADDRMAP4_ADDRMAP_COL_B10_MSK:u32 = 0x1fu32 << UDDRC_ADDRMAP4_ADDRMAP_COL_B10_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP4_ADDRMAP_COL_B10 (value:u32) -> u32 {
    UDDRC_ADDRMAP4_ADDRMAP_COL_B10_MSK & ((value) << UDDRC_ADDRMAP4_ADDRMAP_COL_B10_POS)
}

/* (UDDRC_ADDRMAP4)
* - Full bus width mode: Selects the HIF address bit used as column address
bit 13 (11 in LPDDR2/LPDDR3 mode).
* - Half bus width mode: Unused. To make it unused, this should be tied
to 4'hF.
* - Quarter bus width mode: Unused. To make it unused, this must be tied
to 4'hF.
* Valid Range: 0 to 7, x, and 31. x indicate a valid value in inline ECC
* configuration.
* Internal Base: 11
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 31 and then this column address bit is set to 0.
*
* Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved
* for indicating auto-precharge, and hence no source address bit can be mapped
* to column address bit 10.
* In LPDDR2/LPDDR3, there is a dedicated bit for auto-precharge in the CA bus
* and hence column bit 10 is used.
*
* In Inline ECC configuration (MEMC_INLINE_ECC=1) and ECC is enabled
* (ECCCFG0.ecc_mode>0), the highest 3 column address bits must map to the
* highest 3 valid HIF address bits.
*
* If column bit 11 is the highest column address bit, it must map to the
* highest valid HIF address bit.
* (x = the highest valid HIF address bit - internal base)
*
* If column bit 11 is the second highest column address bit, it must map to the
* second highest valid HIF address bit.
* (x = the highest valid HIF address bit - 1 - internal base)
*
* If column bit 11 is the third highest column address bit, it must map to the
* third highest valid HIF address bit.
* (x = the highest valid HIF address bit - 2 - internal base)
*
* if it is unused, set to 31.
*/
pub const UDDRC_ADDRMAP4_ADDRMAP_COL_B11_POS:u32 = 8;
pub const UDDRC_ADDRMAP4_ADDRMAP_COL_B11_MSK:u32 = 0x1fu32 << UDDRC_ADDRMAP4_ADDRMAP_COL_B11_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP4_ADDRMAP_COL_B11 (value:u32) -> u32 {
    UDDRC_ADDRMAP4_ADDRMAP_COL_B11_MSK & ((value) << UDDRC_ADDRMAP4_ADDRMAP_COL_B11_POS)
}

/* -------- UDDRC_ADDRMAP5 : (UDDRC_REGS Offset: 0x214)
* Address Map Register 5 --------
*/
/* (UDDRC_ADDRMAP5) Selects the HIF address bits used as row address bit 0.
* Valid Range: 0 to 11
* Internal Base: 6
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field.
*/
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B0_POS:u32 = 0;
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B0_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP5_ADDRMAP_ROW_B0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP5_ADDRMAP_ROW_B0 (value:u32) -> u32 {
    UDDRC_ADDRMAP5_ADDRMAP_ROW_B0_MSK & ((value) << UDDRC_ADDRMAP5_ADDRMAP_ROW_B0_POS)
}

/* (UDDRC_ADDRMAP5) Selects the HIF address bits used as row address bit 1.
* Valid Range: 0 to 11
* Internal Base: 7
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field.
*/
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B1_POS:u32 = 8;
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B1_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP5_ADDRMAP_ROW_B1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP5_ADDRMAP_ROW_B1 (value:u32) -> u32 {
    UDDRC_ADDRMAP5_ADDRMAP_ROW_B1_MSK & ((value) << UDDRC_ADDRMAP5_ADDRMAP_ROW_B1_POS)
}

/* (UDDRC_ADDRMAP5) Selects the HIF address bits used as row address
* bits 2 to 10.
* Valid Range: 0 to 11, and 15
* Internal Base: 8 (for row address bit 2), 9 (for row address bit 3),
* 10 (for row address bit 4) etc increasing to 16 (for row address bit 10)
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. When value 15 is used
* the values of row address bits 2 to 10 are defined by registers
* ADDRMAP9, ADDRMAP10, ADDRMAP11.
*/
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10_POS:u32 = 16;
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10 (value:u32) -> u32 {
    UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10_MSK & ((value) << UDDRC_ADDRMAP5_ADDRMAP_ROW_B2_10_POS)
}

/* (UDDRC_ADDRMAP5) Selects the HIF address bit used as row address bit 11.
* Valid Range: 0 to 11, and 15
* Internal Base: 17
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 15 and then row address bit 11 is set to 0.
*/
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B11_POS:u32 = 24;
pub const UDDRC_ADDRMAP5_ADDRMAP_ROW_B11_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP5_ADDRMAP_ROW_B11_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP5_ADDRMAP_ROW_B11 (value:u32) -> u32 {
    UDDRC_ADDRMAP5_ADDRMAP_ROW_B11_MSK & ((value) << UDDRC_ADDRMAP5_ADDRMAP_ROW_B11_POS)
}

/* -------- UDDRC_ADDRMAP6 : (UDDRC_REGS Offset: 0x218)
Address Map Register 6 --------
*/
/* (UDDRC_ADDRMAP6) Selects the HIF address bit used as row address bit 12.
* Valid Range: 0 to 11, and 15
* Internal Base: 18
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 15 and then row address bit 12 is set to 0.
*/
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B12_POS:u32 = 0;
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B12_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP6_ADDRMAP_ROW_B12_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP6_ADDRMAP_ROW_B12 (value:u32) -> u32 {
    UDDRC_ADDRMAP6_ADDRMAP_ROW_B12_MSK & ((value) << UDDRC_ADDRMAP6_ADDRMAP_ROW_B12_POS)
}

/* (UDDRC_ADDRMAP6) Selects the HIF address bit used as row address bit 13.
* Valid Range: 0 to 11, and 15
* Internal Base: 19
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 15 and then row address bit 13 is set to 0.
*/
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B13_POS:u32 = 8;
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B13_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP6_ADDRMAP_ROW_B13_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP6_ADDRMAP_ROW_B13 (value:u32) -> u32 {
    UDDRC_ADDRMAP6_ADDRMAP_ROW_B13_MSK & ((value) << UDDRC_ADDRMAP6_ADDRMAP_ROW_B13_POS)
}

/* (UDDRC_ADDRMAP6) Selects the HIF address bit used as row address bit 14.
* Valid Range: 0 to 11, and 15
* Internal Base: 20
* The selected HIF address bit is determined by adding the internal base to the
* value of this field.
* If unused, set to 15 and then row address bit 14 is set to 0.
*/
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B14_POS:u32 = 16;
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B14_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP6_ADDRMAP_ROW_B14_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP6_ADDRMAP_ROW_B14 (value:u32) -> u32 {
    UDDRC_ADDRMAP6_ADDRMAP_ROW_B14_MSK & ((value) << UDDRC_ADDRMAP6_ADDRMAP_ROW_B14_POS)
}
/* (UDDRC_ADDRMAP6) Selects the HIF address bit used as row address bit 15.
* Valid Range: 0 to 11, and 15
* Internal Base: 21
* The selected HIF address bit is determined by adding the internal base
* to the value of this field.
* If unused, set to 15 and then row address bit 15 is set to 0.
*/
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B15_POS:u32 = 24;
pub const UDDRC_ADDRMAP6_ADDRMAP_ROW_B15_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP6_ADDRMAP_ROW_B15_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP6_ADDRMAP_ROW_B15 (value:u32) -> u32 {
    UDDRC_ADDRMAP6_ADDRMAP_ROW_B15_MSK & ((value) << UDDRC_ADDRMAP6_ADDRMAP_ROW_B15_POS)
}

/* (UDDRC_ADDRMAP6)
* Set this to 1 if there is an LPDDR3 SDRAM 6Gb or 12Gb device in use.
* - 1 - LPDDR3 SDRAM 6Gb/12Gb device in use. Every address having
* row[14:13]==2'b11 is considered as invalid
* - 0 - non-LPDDR3 6Gb/12Gb device in use. All addresses are valid
* Present only in designs configured to support LPDDR3.
*/
pub const UDDRC_ADDRMAP6_LPDDR3_6GB_12GB:u32 = 0x1u32 << 31;

pub const UDDRC_ADDRMAP7_ADDRMAP_ROW_B16_POS:u32 = 0;
pub const UDDRC_ADDRMAP7_ADDRMAP_ROW_B16_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP7_ADDRMAP_ROW_B16_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP7_ADDRMAP_ROW_B16 (value:u32) -> u32 {
    UDDRC_ADDRMAP7_ADDRMAP_ROW_B16_MSK & ((value) << UDDRC_ADDRMAP7_ADDRMAP_ROW_B16_POS)
}

pub const UDDRC_ADDRMAP7_ADDRMAP_ROW_B17_POS:u32 = 8;
pub const UDDRC_ADDRMAP7_ADDRMAP_ROW_B17_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP7_ADDRMAP_ROW_B17_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP7_ADDRMAP_ROW_B17 (value:u32) -> u32 {
    UDDRC_ADDRMAP7_ADDRMAP_ROW_B17_MSK & ((value) << UDDRC_ADDRMAP7_ADDRMAP_ROW_B17_POS)
}


pub const UDDRC_ADDRMAP8_ADDRMAP_BG_B0_POS:u32 = 0;
pub const UDDRC_ADDRMAP8_ADDRMAP_BG_B0_MSK:u32 = 0x3fu32 << UDDRC_ADDRMAP8_ADDRMAP_BG_B0_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP8_ADDRMAP_BG_B0 (value:u32) -> u32 {
    UDDRC_ADDRMAP8_ADDRMAP_BG_B0_MSK & ((value) << UDDRC_ADDRMAP8_ADDRMAP_BG_B0_POS)
}

pub const UDDRC_ADDRMAP8_ADDRMAP_BG_B1_POS:u32 = 8;
pub const UDDRC_ADDRMAP8_ADDRMAP_BG_B1_MSK:u32 = 0x3fu32 << UDDRC_ADDRMAP8_ADDRMAP_BG_B1_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP8_ADDRMAP_BG_B1 (value:u32) -> u32 {
    UDDRC_ADDRMAP8_ADDRMAP_BG_B1_MSK & ((value) << UDDRC_ADDRMAP8_ADDRMAP_BG_B1_POS)
}


/* -------- UDDRC_ADDRMAP9 : (UDDRC_REGS Offset: 0x224)
* Address Map Register 9 --------
*/
/* (UDDRC_ADDRMAP9) Selects the HIF address bits used as row address bit 2.
* Valid Range: 0 to 11
* Internal Base: 8
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B2_POS:u32 = 0;
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B2_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP9_ADDRMAP_ROW_B2_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP9_ADDRMAP_ROW_B2 (value:u32) -> u32 {
    UDDRC_ADDRMAP9_ADDRMAP_ROW_B2_MSK & ((value) << UDDRC_ADDRMAP9_ADDRMAP_ROW_B2_POS)
}

/* (UDDRC_ADDRMAP9) Selects the HIF address bits used as row address bit 3.
* Valid Range: 0 to 11
* Internal Base: 9
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B3_POS:u32 = 8;
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B3_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP9_ADDRMAP_ROW_B3_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP9_ADDRMAP_ROW_B3 (value:u32) -> u32 {
    UDDRC_ADDRMAP9_ADDRMAP_ROW_B3_MSK & ((value) << UDDRC_ADDRMAP9_ADDRMAP_ROW_B3_POS)
}

/* (UDDRC_ADDRMAP9) Selects the HIF address bits used as row address bit 4.
* Valid Range: 0 to 11
* Internal Base: 10
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B4_POS:u32 = 16;
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B4_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP9_ADDRMAP_ROW_B4_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP9_ADDRMAP_ROW_B4 (value:u32) -> u32 {
    UDDRC_ADDRMAP9_ADDRMAP_ROW_B4_MSK & ((value) << UDDRC_ADDRMAP9_ADDRMAP_ROW_B4_POS)
}

/* (UDDRC_ADDRMAP9) Selects the HIF address bits used as row address bit 5.
* Valid Range: 0 to 11
* Internal Base: 11
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B5_POS:u32 = 24;
pub const UDDRC_ADDRMAP9_ADDRMAP_ROW_B5_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP9_ADDRMAP_ROW_B5_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP9_ADDRMAP_ROW_B5 (value:u32) -> u32 {
    UDDRC_ADDRMAP9_ADDRMAP_ROW_B5_MSK & ((value) << UDDRC_ADDRMAP9_ADDRMAP_ROW_B5_POS)
}

/* -------- UDDRC_ADDRMAP10 : (UDDRC_REGS Offset: 0x228)
* Address Map Register 10 --------
*/
/* (UDDRC_ADDRMAP10) Selects the HIF address bits used as row address bit 6.
* Valid Range: 0 to 11
* Internal Base: 12
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B6_POS:u32 = 0;
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B6_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP10_ADDRMAP_ROW_B6_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP10_ADDRMAP_ROW_B6 (value:u32) -> u32 {
    UDDRC_ADDRMAP10_ADDRMAP_ROW_B6_MSK & ((value) << UDDRC_ADDRMAP10_ADDRMAP_ROW_B6_POS)
}

/* (UDDRC_ADDRMAP10) Selects the HIF address bits used as row address bit 7.
* Valid Range: 0 to 11
* Internal Base: 13
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B7_POS:u32 = 8;
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B7_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP10_ADDRMAP_ROW_B7_POS;


#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP10_ADDRMAP_ROW_B7 (value:u32) -> u32 {
    UDDRC_ADDRMAP10_ADDRMAP_ROW_B7_MSK & ((value) << UDDRC_ADDRMAP10_ADDRMAP_ROW_B7_POS)
}

/* (UDDRC_ADDRMAP10) Selects the HIF address bits used as row address bit 8.
* Valid Range: 0 to 11
* Internal Base: 14
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B8_POS:u32 = 16;
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B8_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP10_ADDRMAP_ROW_B8_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP10_ADDRMAP_ROW_B8 (value:u32) -> u32 {
    UDDRC_ADDRMAP10_ADDRMAP_ROW_B8_MSK & ((value) << UDDRC_ADDRMAP10_ADDRMAP_ROW_B8_POS)
}

/* (UDDRC_ADDRMAP10) Selects the HIF address bits used as row address bit 9.
* Valid Range: 0 to 11
* Internal Base: 15
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B9_POS:u32 = 24;
pub const UDDRC_ADDRMAP10_ADDRMAP_ROW_B9_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP10_ADDRMAP_ROW_B9_POS;

#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP10_ADDRMAP_ROW_B9 (value:u32) -> u32 {
    UDDRC_ADDRMAP10_ADDRMAP_ROW_B9_MSK & ((value) << UDDRC_ADDRMAP10_ADDRMAP_ROW_B9_POS)
}

/* -------- UDDRC_ADDRMAP11 : (UDDRC_REGS Offset: 0x22C)
* Address Map Register 11 --------
*/
/* (UDDRC_ADDRMAP11) Selects the HIF address bits used as row address bit 10.
* Valid Range: 0 to 11
* Internal Base: 16
* The selected HIF address bit for each of the row address bits is determined
* by adding the internal base to the value of this field. This register field
* is used only when ADDRMAP5.addrmap_row_b2_10 is set to value 15.
*/
pub const UDDRC_ADDRMAP11_ADDRMAP_ROW_B10_POS:u32 = 0;
pub const UDDRC_ADDRMAP11_ADDRMAP_ROW_B10_MSK:u32 = 0xfu32 << UDDRC_ADDRMAP11_ADDRMAP_ROW_B10_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ADDRMAP11_ADDRMAP_ROW_B10 (value:u32) -> u32 {
    UDDRC_ADDRMAP11_ADDRMAP_ROW_B10_MSK & ((value) << UDDRC_ADDRMAP11_ADDRMAP_ROW_B10_POS)
}

/* -------- UDDRC_ODTCFG : (UDDRC_REGS Offset: 0x240)
* ODT Configuration Register --------
*/
/* (UDDRC_ODTCFG)
* The delay, in DFI PHY clock cycles, from issuing a read command to setting
* ODT values associated with that command. ODT setting must remain constant
* for the entire time that DQS is driven by the uMCTL2.
*
* Recommended values:
* DDR2:
* - CL + AL - 4 (not DDR2-1066),  CL + AL - 5 (DDR2-1066)
* If (CL + AL - 4 < 0),  uMCTL2 does not support ODT for read operation.
*
* DDR3:
* - CL - CWL
* DDR4:
* - CL - CWL - RD_PREAMBLE + WR_PREAMBLE + DFITMG1.dfi_t_cmd_lat
* (to adjust for CAL mode)
*
*  WR_PREAMBLE = 1 (1tCK write preamble),  2 (2tCK write preamble)
*  RD_PREAMBLE = 1 (1tCK write preamble),  2 (2tCK write preamble)
*  If (CL - CWL - RD_PREAMBLE + WR_PREAMBLE) < 0,  uMCTL2 does not support
* ODT for read operation.
*
* LPDDR3:
* - RL + RD(tDQSCK(min)/tCK) - 1 - RU(tODTon(max)/tCK)
*/

pub const UDDRC_ODTCFG_RD_ODT_DELAY_POS:u32 = 2;
pub const UDDRC_ODTCFG_RD_ODT_DELAY_MSK:u32 = 0x1fu32 << UDDRC_ODTCFG_RD_ODT_DELAY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ODTCFG_RD_ODT_DELAY (value:u32) -> u32 {
    UDDRC_ODTCFG_RD_ODT_DELAY_MSK & ((value) << UDDRC_ODTCFG_RD_ODT_DELAY_POS)
}

/* (UDDRC_ODTCFG)
* DFI PHY clock cycles to hold ODT for a read command.
* The minimum supported value is 2.
* Recommended values:
*
* DDR2:
* - BL8: 0x6 (not DDR2-1066),  0x7 (DDR2-1066)
* - BL4: 0x4 (not DDR2-1066),  0x5 (DDR2-1066)
* DDR3:
* - BL8 - 0x6
* DDR4:
* - BL8: 5 + RD_PREAMBLE
* RD_PREAMBLE = 1 (1tCK write preamble),  2 (2tCK write preamble)
* LPDDR3:
* - BL8:  5 + RU(tDQSCK(max)/tCK) - RD(tDQSCK(min)/tCK) + RU(tODTon(max)/tCK)
*/
pub const UDDRC_ODTCFG_RD_ODT_HOLD_POS:u32 = 8;
pub const UDDRC_ODTCFG_RD_ODT_HOLD_MSK:u32 = 0xfu32 << UDDRC_ODTCFG_RD_ODT_HOLD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ODTCFG_RD_ODT_HOLD (value:u32) -> u32 {
    UDDRC_ODTCFG_RD_ODT_HOLD_MSK & ((value) << UDDRC_ODTCFG_RD_ODT_HOLD_POS)
}

/* (UDDRC_ODTCFG)
* The delay, in DFI PHY clock cycles, from issuing a write command to setting
* ODT values associated with that command. ODT setting must remain constant
* for the entire time that DQS is driven by the uMCTL2.
* Recommended values:
*
* DDR2:
* - CWL + AL - 3 (DDR2-400/533/667),  CWL + AL - 4 (DDR2-800),
*  CWL + AL - 5 (DDR2-1066)
* If (CWL + AL - 3  < 0),  uMCTL2 does not support ODT for write operation.
* DDR3:
* - 0x0
* DDR4:
* - DFITMG1.dfi_t_cmd_lat (to adjust for CAL mode)
* LPDDR3:
* - WL - 1 - RU(tODTon(max)/tCK))
*/
pub const UDDRC_ODTCFG_WR_ODT_DELAY_POS:u32 = 16;
pub const UDDRC_ODTCFG_WR_ODT_DELAY_MSK:u32 = 0x1fu32 << UDDRC_ODTCFG_WR_ODT_DELAY_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ODTCFG_WR_ODT_DELAY (value:u32) -> u32 {
    UDDRC_ODTCFG_WR_ODT_DELAY_MSK & ((value) << UDDRC_ODTCFG_WR_ODT_DELAY_POS)
}

/* (UDDRC_ODTCFG)
* DFI PHY clock cycles to hold ODT for a write command.
* The minimum supported value is 2.
* Recommended values:
* DDR2:
* - BL8:  0x5 (DDR2-400/533/667),  0x6 (DDR2-800),  0x7 (DDR2-1066)
* - BL4:  0x3 (DDR2-400/533/667),  0x4 (DDR2-800),  0x5 (DDR2-1066)
* DDR3:
* - BL8: 0x6
* DDR4:
* - BL8: 5 + WR_PREAMBLE + CRC_MODE
* WR_PREAMBLE = 1 (1tCK write preamble),  2 (2tCK write preamble)
* CRC_MODE = 0 (not CRC mode),  1 (CRC mode)
* LPDDR3:
* - BL8: 7 + RU(tODTon(max)/tCK)
*/
pub const UDDRC_ODTCFG_WR_ODT_HOLD_POS:u32 = 24;
pub const UDDRC_ODTCFG_WR_ODT_HOLD_MSK:u32 = 0xfu32 << UDDRC_ODTCFG_WR_ODT_HOLD_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_ODTCFG_WR_ODT_HOLD (value:u32) -> u32 {
    UDDRC_ODTCFG_WR_ODT_HOLD_MSK & ((value) << UDDRC_ODTCFG_WR_ODT_HOLD_POS)
}

/* -------- UDDRC_ODTMAP : (UDDRC_REGS Offset: 0x244)
* ODT/Rank Map Register --------
*/
/* (UDDRC_ODTMAP) Indicates which remote ODTs must be turned on during a write
* to rank 0.
* Each rank has a remote ODT (in the SDRAM) which can be turned on by setting
* the appropriate bit here.
* Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the
* LSB, etc.
* For each rank, set its bit to 1 to enable its ODT.
*/
pub const UDDRC_ODTMAP_RANK0_WR_ODT:u32 = 0x1u32 << 0;

/* (UDDRC_ODTMAP) Indicates which remote ODTs must be turned on during a read
* from rank 0.
* Each rank has a remote ODT (in the SDRAM) which can be turned on by setting
* the appropriate bit here.
* Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB,
* etc.
* For each rank, set its bit to 1 to enable its ODT.
*/
pub const UDDRC_ODTMAP_RANK0_RD_ODT:u32 = 0x1u32 << 4;

/* -------- UDDRC_SCHED : (UDDRC_REGS Offset: 0x250)
* Scheduler Control Register --------
*/
/* (UDDRC_SCHED) Active low signal. When asserted ('0'), all incoming
* transactions are forced to low priority. This implies that all
* High Priority Read (HPR) and Variable Priority Read commands (VPR) will be
* treated as Low Priority Read (LPR) commands. On the write side, all
* Variable Priority Write (VPW) commands will be treated as
* Normal Priority Write (NPW) commands. Forcing the incoming transactions to
* low priority implicitly turns off Bypass path for read commands.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_SCHED_FORCE_LOW_PRI_N:u32 = 0x1u32 << 0;

/* (UDDRC_SCHED) If set then the bank selector prefers writes over reads.
* FOR DEBUG ONLY.
*/
pub const UDDRC_SCHED_PREFER_WRITE:u32 = 0x1u32 << 1;

/* (UDDRC_SCHED) If true, bank is kept open only while there are page hit
* transactions available in the CAM to that bank. The last read or write
* command in the CAM with a bank and page hit will be executed with
* auto-precharge if SCHED1.pageclose_timer=0. Even if this register set to 1
* and SCHED1.pageclose_timer is set to 0, explicit precharge
* (and not auto-precharge) may be issued in some cases where there is a mode
* switch between Write and Read or between LPR and HPR. The Read and Write
* commands that are executed as part of the ECC scrub requests are also
* executed without auto-precharge.
*
* If false, the bank remains open until there is a need to close it
* (to open a different page, or for page timeout or refresh timeout)
* - also known as open page policy. The open page policy can be overridden
* by setting the per-command-autopre bit on the HIF interface (hif_cmd_autopre).
* The pageclose feature provids a midway between Open and Close page policies.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_SCHED_PAGECLOSE:u32 = 0x1u32 << 2;

/* (UDDRC_SCHED) Number of entries in the low priority transaction store
* is this value + 1.
* (MEMC_NO_OF_ENTRY - (SCHED.lpr_num_entries + 1)) is the number of entries
* available for the high priority transaction store.
* Setting this to maximum value allocates all entries to low priority
* transaction store.
* Setting this to 0 allocates 1 entry to low priority transaction store and
* the rest to high priority transaction store.
* Note: In ECC configurations, the numbers of write and low priority read
* credits issued is one less than in the non-ECC case.  One entry each is
* reserved in the write and low-priority read CAMs for storing the RMW requests
* arising out of single bit error correction RMW operation.
*/
pub const UDDRC_SCHED_LPR_NUM_ENTRIES_POS:u32 = 8;
pub const UDDRC_SCHED_LPR_NUM_ENTRIES_MSK:u32 = 0x1fu32 << UDDRC_SCHED_LPR_NUM_ENTRIES_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_SCHED_LPR_NUM_ENTRIES (value:u32) -> u32 {
    UDDRC_SCHED_LPR_NUM_ENTRIES_MSK & ((value) << UDDRC_SCHED_LPR_NUM_ENTRIES_POS)
}

/* (UDDRC_SCHED) UNUSED */
pub const UDDRC_SCHED_GO2CRITICAL_HYSTERESIS_POS:u32 = 16;
pub const UDDRC_SCHED_GO2CRITICAL_HYSTERESIS_MSK:u32 = 0xffu32 << UDDRC_SCHED_GO2CRITICAL_HYSTERESIS_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_SCHED_GO2CRITICAL_HYSTERESIS (value:u32) -> u32 {
    UDDRC_SCHED_GO2CRITICAL_HYSTERESIS_MSK & ((value) << UDDRC_SCHED_GO2CRITICAL_HYSTERESIS_POS)
}

/* (UDDRC_SCHED) When the preferred transaction store is empty for these many
* clock cycles, switch to the alternate transaction store if it is non-empty.
* The read transaction store (both high and low priority) is the default
* preferred transaction store and the write transaction store is the
* alternative store.
* When prefer write over read is set this is reversed.
* 0x0 is a legal value for this register. When set to 0x0, the transaction
* store switching will happen immediately when the switching conditions
* become true.
* FOR PERFORMANCE ONLY
*/
pub const UDDRC_SCHED_RDWR_IDLE_GAP_POS:u32 = 24;
pub const UDDRC_SCHED_RDWR_IDLE_GAP_MSK:u32 = 0x7fu32 << UDDRC_SCHED_RDWR_IDLE_GAP_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_SCHED_RDWR_IDLE_GAP (value:u32) -> u32 {
    UDDRC_SCHED_RDWR_IDLE_GAP_MSK & ((value) << UDDRC_SCHED_RDWR_IDLE_GAP_POS)
}

/* -------- UDDRC_SCHED1 : (UDDRC_REGS Offset: 0x254)
* Scheduler Control Register 1 --------
*/
/* (UDDRC_SCHED1) This field works in conjunction with SCHED.pageclose.
* It only has meaning if SCHED.pageclose==1.
*
* If SCHED.pageclose==1 and pageclose_timer==0, then an auto-precharge
* may be scheduled for last read or write command in the CAM with a bank
* and page hit.
* Note, sometimes an explicit precharge is scheduled instead of the
* auto-precharge. See SCHED.pageclose for details of when this may happen.
*
* If SCHED.pageclose==1 and pageclose_timer>0, then an auto-precharge is not
* scheduled for last read or write command in the CAM with a bank and page hit.
* Instead, a timer is started, with pageclose_timer as the initial value.
* There is a timer on a per bank basis.
* The timer decrements unless the next read or write in the CAM to a bank is a
* page hit. It gets reset to pageclose_timer value if the next read or write
* in the CAM to a bank is a page hit. Once the timer has reached zero,
* an explcit precharge will be attempted to be scheduled.
*/
pub const UDDRC_SCHED1_PAGECLOSE_TIMER_POS:u32 = 0;
pub const UDDRC_SCHED1_PAGECLOSE_TIMER_MSK:u32 = 0xffu32 << UDDRC_SCHED1_PAGECLOSE_TIMER_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_SCHED1_PAGECLOSE_TIMER (value:u32) -> u32 {
    UDDRC_SCHED1_PAGECLOSE_TIMER_MSK & ((value) << UDDRC_SCHED1_PAGECLOSE_TIMER_POS)
}

/* -------- UDDRC_PERFHPR1 : (UDDRC_REGS Offset: 0x25C)
* High Priority Read CAM Register 1 --------
*/
/* (UDDRC_PERFHPR1) Number of DFI clocks that the HPR queue can be starved
* before it goes critical. The minimum valid functional value for this register
* is 0x1. Programming it to 0x0 will disable the starvation functionality;
* during normal operation, this function should not be disabled as it will
* cause excessive latencies.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFHPR1_HPR_MAX_STARVE_POS:u32 = 0;
pub const UDDRC_PERFHPR1_HPR_MAX_STARVE_MSK:u32 = 0xffffu32 << UDDRC_PERFHPR1_HPR_MAX_STARVE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFHPR1_HPR_MAX_STARVE (value:u32) -> u32 {
    UDDRC_PERFHPR1_HPR_MAX_STARVE_MSK & ((value) << UDDRC_PERFHPR1_HPR_MAX_STARVE_POS)
}

/* (UDDRC_PERFHPR1)
* Number of transactions that are serviced once the HPR queue goes critical
* is the smaller of:
* - (a) This number
* - (b) Number of transactions available.
* Unit: Transaction.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH_POS:u32 = 24;
pub const UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH_MSK:u32 = 0xffu32 << UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH (value:u32) -> u32 {
    UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH_MSK & ((value) << UDDRC_PERFHPR1_HPR_XACT_RUN_LENGTH_POS)
}

/* -------- UDDRC_PERFLPR1 : (UDDRC_REGS Offset: 0x264)
* Low Priority Read CAM Register 1 --------
*/
/* (UDDRC_PERFLPR1) Number of DFI clocks that the LPR queue can be starved
* before it goes critical. The minimum valid functional value for this register
* is 0x1. Programming it to 0x0 will disable the starvation functionality;
* during normal operation, this function should not be disabled as it will
* cause excessive latencies.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFLPR1_LPR_MAX_STARVE_POS:u32 = 0;
pub const UDDRC_PERFLPR1_LPR_MAX_STARVE_MSK:u32 = 0xffffu32 << UDDRC_PERFLPR1_LPR_MAX_STARVE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFLPR1_LPR_MAX_STARVE (value:u32) -> u32 {
    UDDRC_PERFLPR1_LPR_MAX_STARVE_MSK & ((value) << UDDRC_PERFLPR1_LPR_MAX_STARVE_POS)
}

/* (UDDRC_PERFLPR1)
* Number of transactions that are serviced once the LPR queue goes critical
* is the smaller of:
* - (a) This number
* - (b) Number of transactions available.
* Unit: Transaction.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH_POS:u32 = 24;
pub const UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH_MSK:u32 = 0xffu32 << UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH (value:u32) -> u32 {
    UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH_MSK & ((value) << UDDRC_PERFLPR1_LPR_XACT_RUN_LENGTH_POS)
}
/* -------- UDDRC_PERFWR1 : (UDDRC_REGS Offset: 0x26C)
* Write CAM Register 1 --------
*/
/* (UDDRC_PERFWR1) Number of DFI clocks that the WR queue can be starved before
* it goes critical. The minimum valid functional value for this register is 0x1.
* Programming it to 0x0 will disable the starvation functionality;
* during normal operation, this function should not be disabled as it will
* cause excessive latencies.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFWR1_W_MAX_STARVE_POS:u32 = 0;
pub const UDDRC_PERFWR1_W_MAX_STARVE_MSK:u32 = 0xffffu32 << UDDRC_PERFWR1_W_MAX_STARVE_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFWR1_W_MAX_STARVE (value:u32) -> u32 {
    UDDRC_PERFWR1_W_MAX_STARVE_MSK & ((value) << UDDRC_PERFWR1_W_MAX_STARVE_POS)
}

/* (UDDRC_PERFWR1)
* Number of transactions that are serviced once the WR queue goes critical
* is the smaller of:
* - (a) This number
* - (b) Number of transactions available.
* Unit: Transaction.
* FOR PERFORMANCE ONLY.
*/
pub const UDDRC_PERFWR1_W_XACT_RUN_LENGTH_POS:u32 = 24;
pub const UDDRC_PERFWR1_W_XACT_RUN_LENGTH_MSK:u32 = 0xffu32 << UDDRC_PERFWR1_W_XACT_RUN_LENGTH_POS;
#[allow(non_snake_case)]
pub const fn UDDRC_PERFWR1_W_XACT_RUN_LENGTH (value:u32) -> u32 {
    UDDRC_PERFWR1_W_XACT_RUN_LENGTH_MSK & ((value) << UDDRC_PERFWR1_W_XACT_RUN_LENGTH_POS)
}

/* -------- UDDRC_DBG0 : (UDDRC_REGS Offset: 0x300)
* Debug Register 0 --------
*/
/* (UDDRC_DBG0) When 1, disable write combine.
* FOR DEBUG ONLY
*/
/* (UDDRC_DBG0) Only present in designs supporting read bypass.
* When 1, disable bypass path for high priority read page hits
* FOR DEBUG ONLY.
*/
pub const UDDRC_DBG0_DIS_WC:u32 = 0x1u32 << 0;

/* (UDDRC_DBG0) Only present in designs supporting activate bypass.
* When 1, disable bypass path for high priority read activates
* FOR DEBUG ONLY.
*/
pub const UDDRC_DBG0_DIS_RD_BYPASS:u32 = 0x1u32 << 1;
pub const UDDRC_DBG0_DIS_ACT_BYPASS:u32 = 0x1u32 << 2;

/* (UDDRC_DBG0) When this is set to '0', auto-precharge is disabled for the
* flushed command in a collision case. Collision cases are write followed by
* read to same address, read followed by write to same address,
* or write followed by write to same address with DBG0.dis_wc bit = 1
* (where same address comparisons exclude the two address bits representing
* critical word).
* FOR DEBUG ONLY.
*/
pub const UDDRC_DBG0_DIS_COLLISION_PAGE_OPT:u32 = 0x1u32 << 4;
/* -------- UDDRC_DBG1 : (UDDRC_REGS Offset: 0x304)
* Debug Register 1 --------
*/
/* (UDDRC_DBG1) When 1, uMCTL2 will not de-queue any transactions from the CAM.
* Bypass is also disabled. All transactions are queued in the CAM. No reads or
* writes are issued to SDRAM as long as this is asserted.
* This bit may be used to prevent reads or writes being issued by the uMCTL2,
* which makes it safe to modify certain register fields associated with
* reads and writes (see User Guide for details).  After setting this bit,
* it is strongly recommended to poll DBGCAM.wr_data_pipeline_empty and
* DBGCAM.rd_data_pipeline_empty, before making changes to any registers which
* affect reads and writes.
* This will ensure that the relevant logic in the DDRC is idle.
* This bit is intended to be switched on-the-fly.
*/
pub const UDDRC_DBG1_DIS_DQ:u32 = 0x1u32 << 0;

/* (UDDRC_DBG1) When 1, uMCTL2 asserts the HIF command signal hif_cmd_stall.
* uMCTL2 will ignore the hif_cmd_valid and all other associated request signals.
* This bit is intended to be switched on-the-fly.
*/
pub const UDDRC_DBG1_DIS_HIF:u32 = 0x1u32 << 1;

/* -------- UDDRC_DBGCAM : (UDDRC_REGS Offset: 0x308)
* CAM Debug Register --------
*/
/* (UDDRC_DBGCAM) High priority read queue depth
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_HPR_Q_DEPTH_POS:u32 = 0;
pub const UDDRC_DBGCAM_DBG_HPR_Q_DEPTH_MSK:u32 = 0x3fu32 << UDDRC_DBGCAM_DBG_HPR_Q_DEPTH_POS;
/* (UDDRC_DBGCAM) Low priority read queue depth
* The last entry of Lpr queue is reserved for ECC SCRUB operation.
* This entry is not included in the calculation of the queue depth.
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_LPR_Q_DEPTH_POS:u32 = 8;
pub const UDDRC_DBGCAM_DBG_LPR_Q_DEPTH_MSK:u32 = 0x3fu32 << UDDRC_DBGCAM_DBG_LPR_Q_DEPTH_POS;

/* (UDDRC_DBGCAM) Write queue depth
* The last entry of WR queue is reserved for ECC SCRUB operation.
* This entry is not included in the calculation of the queue depth.
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_W_Q_DEPTH_POS:u32 = 16;
pub const UDDRC_DBGCAM_DBG_W_Q_DEPTH_MSK:u32 = 0x3fu32 << UDDRC_DBGCAM_DBG_W_Q_DEPTH_POS;

/* (UDDRC_DBGCAM) Stall
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_STALL:u32 = 0x1u32 << 24;

/* (UDDRC_DBGCAM) When 1, all the Read command queues and Read data buffers
* inside DDRC are empty. This register is to be used for debug purpose.
* An example use-case scenario: When Controller enters Self-Refresh using the
* Low-Power entry sequence, Controller is expected to have executed all the
* commands in its queues and the write and read data drained.
* Hence this register should be 1 at that time.
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_RD_Q_EMPTY:u32 = 0x1u32 << 25;

/* (UDDRC_DBGCAM) When 1, all the Write command queues and Write data buffers
* inside DDRC are empty. This register is to be used for debug purpose.
* An example use-case scenario: When Controller enters Self-Refresh using the
* Low-Power entry sequence, Controller is expected to have executed all the
* commands in its queues and the write and read data drained.
* Hence this register should be 1 at that time.
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_WR_Q_EMPTY:u32 = 0x1u32 << 26;

/* (UDDRC_DBGCAM) This bit indicates that the read data pipeline on the
* DFI interface is empty.  This register is intended to be polled at least
* twice after setting DBG1.dis_dq, to ensure that all remaining commands/data
* have completed.
*/
pub const UDDRC_DBGCAM_RD_DATA_PIPELINE_EMPTY:u32 = 0x1u32 << 28;

/* (UDDRC_DBGCAM) This bit indicates that the write data pipeline on the
* DFI interface is empty.  This register is intended to be polled at least
* twice after setting DBG1.dis_dq, to ensure that all remaining commands/data
* have completed.
*/
pub const UDDRC_DBGCAM_WR_DATA_PIPELINE_EMPTY:u32 = 0x1u32 << 29;

/* (UDDRC_DBGCAM) Stall for Write channel
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_STALL_WR:u32 = 0x1u32 << 30;

/* (UDDRC_DBGCAM) Stall for Read channel
* FOR DEBUG ONLY
*/
pub const UDDRC_DBGCAM_DBG_STALL_RD:u32 = 0x1u32 << 31;

/* -------- UDDRC_DBGCMD : (UDDRC_REGS Offset: 0x30C)
* Command Debug Register --------
*/
/* (UDDRC_DBGCMD)
* Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to
* rank 0. Writing to this bit causes DBGSTAT.rank0_refresh_busy to be set.
* When DBGSTAT.rank0_refresh_busy is cleared, the command has been stored in
* uMCTL2.
* For 3DS configuration, refresh is sent to rank index 0.
* This operation can be performed only when RFSHCTL3.dis_auto_refresh=1.
* It is recommended NOT to set this register bit if in Init or Deep power-down
* operating modes or Maximum Power Saving Mode.
*/
pub const UDDRC_DBGCMD_RANK0_REFRESH:u32 = 0x1u32 << 0;

/* (UDDRC_DBGCMD)
* Setting this register bit to 1 indicates to the uMCTL2 to issue a
* ZQCS (ZQ calibration short)/MPC(ZQ calibration) command to the SDRAM.
* When this request is stored in the uMCTL2, the bit is automatically cleared.
* This operation can be performed only when ZQCTL0.dis_auto_zq=1. It is
* recommended NOT to set this register bit if in Init operating mode.
* This register bit is ignored when in Self-Refresh(except LPDDR4) and
* SR-Powerdown(LPDDR4) and Deep power-down operating modes and
* Maximum Power Saving Mode.
*/
pub const UDDRC_DBGCMD_ZQ_CALIB_SHORT:u32 = 0x1u32 << 4;

/* (UDDRC_DBGCMD)
* Setting this register bit to 1 indicates to the uMCTL2 to issue a
* dfi_ctrlupd_req to the PHY.  When this request is stored in the uMCTL2,
* the bit is automatically cleared. This operation must only be performed when
* DFIUPD0.dis_auto_ctrlupd=1.
*/
pub const UDDRC_DBGCMD_CTRLUPD:u32 = 0x1u32 << 5;

/* -------- UDDRC_DBGSTAT : (UDDRC_REGS Offset: 0x310)
* Status Debug Register --------
*/
/* (UDDRC_DBGSTAT) SoC core may initiate a rank0_refresh operation
* (refresh operation to rank 0) only if this signal is low. This signal
* goes high in the clock after DBGCMD.rank0_refresh is set to one.
* It goes low when the rank0_refresh operation is stored in the uMCTL2.
* It is recommended not to perform rank0_refresh operations when this signal
* is high.
* - 0 - Indicates that the SoC core can initiate a rank0_refresh operation
* - 1 - Indicates that rank0_refresh operation has not been stored yet in the uMCTL2
*/
pub const UDDRC_DBGSTAT_RANK0_REFRESH_BUSY:u32 = 0x1u32 << 0;

/* (UDDRC_DBGSTAT) SoC core may initiate a ZQCS (ZQ calibration short) operation
* only if this signal is low. This signal goes high in the clock after the
* uMCTL2 accepts the ZQCS request. It goes low when the ZQCS operation is
* initiated in the uMCTL2. It is recommended not to perform ZQCS operations
* when this signal is high.
* - 0 - Indicates that the SoC core can initiate a ZQCS operation
* - 1 - Indicates that ZQCS operation has not been initiated yet in the uMCTL2
*/
pub const UDDRC_DBGSTAT_ZQ_CALIB_SHORT_BUSY:u32 = 0x1u32 << 4;

/* (UDDRC_DBGSTAT) SoC core may initiate a ctrlupd operation only if this
* signal is low. This signal goes high in the clock after the uMCTL2 accepts
* the ctrlupd request. It goes low when the ctrlupd operation is initiated
* in the uMCTL2. It is recommended not to perform ctrlupd operations when this
* signal is high.
* - 0 - Indicates that the SoC core can initiate a ctrlupd operation
* - 1 - Indicates that ctrlupd operation has not been initiated yet in the uMCTL2
*/
pub const UDDRC_DBGSTAT_CTRLUPD_BUSY:u32 = 0x1u32 << 5;

/* -------- UDDRC_SWCTL : (UDDRC_REGS Offset: 0x320)
* Software Register Programming Control Enable --------
*/
/* (UDDRC_SWCTL) Enable quasi-dynamic register programming outside reset.
* Program register to 0 to enable quasi-dynamic programming.
* Set back register to 1 once programming is done.
*/
pub const UDDRC_SWCTL_SW_DONE:u32 = 0x1u32 << 0;

/* -------- UDDRC_SWSTAT : (UDDRC_REGS Offset: 0x324)
* Software Register Programming Control Status --------
*/
/* (UDDRC_SWSTAT) Register programming done. This register is the echo of
* SWCTL.sw_done. Wait for sw_done value 1 to propagate to sw_done_ack
* at the end of the programming sequence to ensure that the correct registers
* values are propagated to the destination clock domains.
*/
pub const UDDRC_SWSTAT_SW_DONE_ACK:u32 = 0x1u32 << 0;

/* -------- UDDRC_POISONCFG : (UDDRC_REGS Offset: 0x36C)
* AXI Poison Configuration Register. Common for all AXI ports --------
*/
/* (UDDRC_POISONCFG) If set to 1, enables SLVERR response for write
* transaction poisoning
*/
pub const UDDRC_POISONCFG_WR_POISON_SLVERR_EN:u32 = 0x1u32 << 0;

/* (UDDRC_POISONCFG) If set to 1, enables interrupts for write
* transaction poisoning
*/
pub const UDDRC_POISONCFG_WR_POISON_INTR_EN:u32 = 0x1u32 << 4;

/* (UDDRC_POISONCFG) Interrupt clear for write transaction poisoning.
* Allow 2/3 clock cycles for correct value to propagate to core logic and
* clear the interrupts.
*/
pub const UDDRC_POISONCFG_WR_POISON_INTR_CLR:u32 = 0x1u32 << 8;

/* (UDDRC_POISONCFG) If set to 1, enables SLVERR response for read
* transaction poisoning
*/
pub const UDDRC_POISONCFG_RD_POISON_SLVERR_EN:u32 = 0x1u32 << 16;

/* (UDDRC_POISONCFG) If set to 1, enables interrupts for read
* transaction poisoning
*/
pub const UDDRC_POISONCFG_RD_POISON_INTR_EN:u32 = 0x1u32 << 20;

/* (UDDRC_POISONCFG) Interrupt clear for read transaction poisoning.
* Allow 2/3 clock cycles for correct value to propagate to core logic and
* clear the interrupts.
*/
pub const UDDRC_POISONCFG_RD_POISON_INTR_CLR:u32 = 0x1u32 << 24;

/* -------- UDDRC_POISONSTAT : (UDDRC_REGS Offset: 0x370)
* AXI Poison Status Register --------
*/
/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 0.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_0:u32 = 0x1u32 << 0;

/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 1.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_1:u32 = 0x1u32 << 1;

/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 2.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_2:u32 = 0x1u32 << 2;

/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 3.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_3:u32 = 0x1u32 << 3;

/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 4.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_4:u32 = 0x1u32 << 4;

/* (UDDRC_POISONSTAT) Write transaction poisoning error interrupt for port 5.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's write address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register wr_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_WR_POISON_INTR_5:u32 = 0x1u32 << 5;

/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 0.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/

pub const UDDRC_POISONSTAT_RD_POISON_INTR_0:u32 = 0x1u32 << 16;
/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 1.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_RD_POISON_INTR_1:u32 = 0x1u32 << 17;

/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 2.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_RD_POISON_INTR_2:u32 = 0x1u32 << 18;

/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 3.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_RD_POISON_INTR_3:u32 = 0x1u32 << 19;

/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 4.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_RD_POISON_INTR_4:u32 = 0x1u32 << 20;

/* (UDDRC_POISONSTAT) Read transaction poisoning error interrupt for port 5.
* This register is a APB clock copy (double register synchronizer) of the
* interrupt asserted when a transaction is poisoned on the corresponding
* AXI port's read address channel. Bit 0 corresponds to Port 0, and so on.
* Interrupt is cleared by register rd_poison_intr_clr, then value propagated
* to APB clock.
*/
pub const UDDRC_POISONSTAT_RD_POISON_INTR_5:u32 = 0x1u32 << 21;

/* } */

/* UMCTL2 main register helpers end */
