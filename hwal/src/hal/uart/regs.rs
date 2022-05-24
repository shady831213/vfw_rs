pub const UART_RX: usize = 0;
pub const UART_TX: usize = 0;

pub const UART_IER: usize = 1;
pub const UART_IER_MSI: usize = 0x08;
pub const UART_IER_RLSI: usize = 0x04;
pub const UART_IER_THRI: usize = 0x02;
pub const UART_IER_RDI: usize = 0x01;

pub const UART_IIR: usize = 2;
pub const UART_IIR_NO_INT: usize = 0x01;
pub const UART_IIR_ID: usize = 0x0e;
pub const UART_IIR_MSI: usize = 0x00;
pub const UART_IIR_THRI: usize = 0x02;
pub const UART_IIR_RDI: usize = 0x04;
pub const UART_IIR_RLSI: usize = 0x06;

pub const UART_IIR_BUSY: usize = 0x07;

pub const UART_IIR_RX_TIMEOUT: usize = 0x0c;
pub const UART_IIR_XOFF: usize = 0x10;
pub const UART_IIR_CTS_RTS_DSR: usize = 0x20;

pub const UART_FCR: usize = 2;
pub const UART_FCR_ENABLE_FIFO: usize = 0x01;
pub const UART_FCR_CLEAR_RCVR: usize = 0x02;
pub const UART_FCR_CLEAR_XMIT: usize = 0x04;
pub const UART_FCR_DMA_SELECT: usize = 0x08;

pub const UART_FCR_R_TRIG_00: usize = 0x00;
pub const UART_FCR_R_TRIG_01: usize = 0x40;
pub const UART_FCR_R_TRIG_10: usize = 0x80;
pub const UART_FCR_R_TRIG_11: usize = 0xc0;
pub const UART_FCR_T_TRIG_00: usize = 0x00;
pub const UART_FCR_T_TRIG_01: usize = 0x10;
pub const UART_FCR_T_TRIG_10: usize = 0x20;
pub const UART_FCR_T_TRIG_11: usize = 0x30;

pub const UART_FCR_TRIGGER_MASK: usize = 0xC0;
pub const UART_FCR_TRIGGER_1: usize = 0x00;
pub const UART_FCR_TRIGGER_4: usize = 0x40;
pub const UART_FCR_TRIGGER_8: usize = 0x80;
pub const UART_FCR_TRIGGER_14: usize = 0xC0;

pub const UART_FCR6_R_TRIGGER_8: usize = 0x00;
pub const UART_FCR6_R_TRIGGER_16: usize = 0x40;
pub const UART_FCR6_R_TRIGGER_24: usize = 0x80;
pub const UART_FCR6_R_TRIGGER_28: usize = 0xC0;
pub const UART_FCR6_T_TRIGGER_16: usize = 0x00;
pub const UART_FCR6_T_TRIGGER_8: usize = 0x10;
pub const UART_FCR6_T_TRIGGER_24: usize = 0x20;
pub const UART_FCR6_T_TRIGGER_30: usize = 0x30;
pub const UART_FCR7_64BYTE: usize = 0x20;

pub const UART_FCR_R_TRIG_SHIFT: usize = 6;
pub const fn uart_fcr_r_trig_bits(x: usize) -> usize {
    (x & UART_FCR_TRIGGER_MASK) >> UART_FCR_R_TRIG_SHIFT
}
pub const UART_FCR_R_TRIG_MAX_STATE: usize = 4;

pub const UART_LCR: usize = 3;
/*
 * Note: if the word length is 5 bits (UART_LCR_WLEN5), then setting
 * UART_LCR_STOP will select 1.5 stop bits, not 2 stop bits.
 */
pub const UART_LCR_DLAB: usize = 0x80;
pub const UART_LCR_SBC: usize = 0x40;
pub const UART_LCR_SPAR: usize = 0x20;
pub const UART_LCR_EPAR: usize = 0x10;
pub const UART_LCR_PARITY: usize = 0x08;
pub const UART_LCR_STOP: usize = 0x04;
pub const UART_LCR_WLEN5: usize = 0x00;
pub const UART_LCR_WLEN6: usize = 0x01;
pub const UART_LCR_WLEN7: usize = 0x02;
pub const UART_LCR_WLEN8: usize = 0x03;

pub const UART_LCR_CONF_MODE_A: usize = UART_LCR_DLAB;
pub const UART_LCR_CONF_MODE_B: usize = 0xBF;

pub const UART_MCR: usize = 4;
pub const UART_MCR_CLKSEL: usize = 0x80;
pub const UART_MCR_TCRTLR: usize = 0x40;
pub const UART_MCR_XONANY: usize = 0x20;
pub const UART_MCR_AFE: usize = 0x20;
pub const UART_MCR_LOOP: usize = 0x10;
pub const UART_MCR_OUT2: usize = 0x08;
pub const UART_MCR_OUT1: usize = 0x04;
pub const UART_MCR_RTS: usize = 0x02;
pub const UART_MCR_DTR: usize = 0x01;

pub const UART_LSR: usize = 5;
pub const UART_LSR_FIFOE: usize = 0x80;
pub const UART_LSR_TEMT: usize = 0x40;
pub const UART_LSR_THRE: usize = 0x20;
pub const UART_LSR_BI: usize = 0x10;
pub const UART_LSR_FE: usize = 0x08;
pub const UART_LSR_PE: usize = 0x04;
pub const UART_LSR_OE: usize = 0x02;
pub const UART_LSR_DR: usize = 0x01;
pub const UART_LSR_BRK_ERROR_BITS: usize = 0x1E;

pub const UART_MSR: usize = 6;
pub const UART_MSR_DCD: usize = 0x80;
pub const UART_MSR_RI: usize = 0x40;
pub const UART_MSR_DSR: usize = 0x20;
pub const UART_MSR_CTS: usize = 0x10;
pub const UART_MSR_DDCD: usize = 0x08;
pub const UART_MSR_TERI: usize = 0x04;
pub const UART_MSR_DDSR: usize = 0x02;
pub const UART_MSR_DCTS: usize = 0x01;
pub const UART_MSR_ANY_DELTA: usize = 0x0F;

pub const UART_SCR: usize = 7;

/*
 * DLAB=1
 */
pub const UART_DLL: usize = 0;
pub const UART_DLM: usize = 1;
pub const UART_DIV_MAX: usize = 0xFFFF;

pub const UART_EFR: usize = 2;
pub const UART_XR_EFR: usize = 9;
pub const UART_EFR_CTS: usize = 0x80;
pub const UART_EFR_RTS: usize = 0x40;
pub const UART_EFR_SCD: usize = 0x20;
pub const UART_EFR_ECB: usize = 0x10;

pub const UART_XON1: usize = 4;
pub const UART_XON2: usize = 5;
pub const UART_XOFF1: usize = 6;
pub const UART_XOFF2: usize = 7;

pub const UART_TI752_TCR: usize = 6;
pub const UART_TI752_TLR: usize = 7;

pub const UART_TRG: usize = 0;

pub const UART_TRG_1: usize = 0x01;
pub const UART_TRG_4: usize = 0x04;
pub const UART_TRG_8: usize = 0x08;
pub const UART_TRG_16: usize = 0x10;
pub const UART_TRG_32: usize = 0x20;
pub const UART_TRG_64: usize = 0x40;
pub const UART_TRG_96: usize = 0x60;
pub const UART_TRG_120: usize = 0x78;
pub const UART_TRG_128: usize = 0x80;

pub const UART_FCTR: usize = 1;
pub const UART_FCTR_RTS_NODELAY: usize = 0x00;
pub const UART_FCTR_RTS_4DELAY: usize = 0x01;
pub const UART_FCTR_RTS_6DELAY: usize = 0x02;
pub const UART_FCTR_RTS_8DELAY: usize = 0x03;
pub const UART_FCTR_IRDA: usize = 0x04;
pub const UART_FCTR_TX_INT: usize = 0x08;
pub const UART_FCTR_TRGA: usize = 0x00;
pub const UART_FCTR_TRGB: usize = 0x10;
pub const UART_FCTR_TRGC: usize = 0x20;
pub const UART_FCTR_TRGD: usize = 0x30;
pub const UART_FCTR_SCR_SWAP: usize = 0x40;
pub const UART_FCTR_RX: usize = 0x00;
pub const UART_FCTR_TX: usize = 0x80;

pub const UART_EMSR: usize = 7;
pub const UART_EMSR_FIFO_COUNT: usize = 0x01;
pub const UART_EMSR_ALT_COUNT: usize = 0x02;

pub const UART_IER_DMAE: usize = 0x80;
pub const UART_IER_UUE: usize = 0x40;
pub const UART_IER_NRZE: usize = 0x20;
pub const UART_IER_RTOIE: usize = 0x10;

pub const UART_IER_PTIME: usize = 0x80;

pub const UART_IIR_TOD: usize = 0x08;

pub const UART_FCR_PXAR1: usize = 0x00;
pub const UART_FCR_PXAR8: usize = 0x40;
pub const UART_FCR_PXAR16: usize = 0x80;
pub const UART_FCR_PXAR32: usize = 0xc0;

pub const UART_ASR: usize = 0x01;
pub const UART_RFL: usize = 0x03;
pub const UART_TFL: usize = 0x04;
pub const UART_ICR: usize = 0x05;

pub const UART_DW_RFL: usize = 0x21;
pub const UART_DW_TFL: usize = 0x20;

pub const UART_ACR: usize = 0x00;
pub const UART_CPR: usize = 0x01;
pub const UART_TCR: usize = 0x02;
pub const UART_CKS: usize = 0x03;
pub const UART_TTL: usize = 0x04;
pub const UART_RTL: usize = 0x05;
pub const UART_FCL: usize = 0x06;
pub const UART_FCH: usize = 0x07;
pub const UART_ID1: usize = 0x08;
pub const UART_ID2: usize = 0x09;
pub const UART_ID3: usize = 0x0A;
pub const UART_REV: usize = 0x0B;
pub const UART_CSR: usize = 0x0C;
pub const UART_NMR: usize = 0x0D;
pub const UART_CTR: usize = 0xFF;

pub const UART_ACR_RXDIS: usize = 0x01;
pub const UART_ACR_TXDIS: usize = 0x02;
pub const UART_ACR_DSRFC: usize = 0x04;
pub const UART_ACR_TLENB: usize = 0x20;
pub const UART_ACR_ICRRD: usize = 0x40;
pub const UART_ACR_ASREN: usize = 0x80;

pub const UART_RSA_BASE: usize = -8isize as usize;

pub const UART_RSA_MSR: usize = UART_RSA_BASE + 0;

pub const UART_RSA_MSR_SWAP: usize = 1 << 0;
pub const UART_RSA_MSR_FIFO: usize = 1 << 2;
pub const UART_RSA_MSR_FLOW: usize = 1 << 3;
pub const UART_RSA_MSR_ITYP: usize = 1 << 4;

pub const UART_RSA_IER: usize = UART_RSA_BASE + 1;

pub const UART_RSA_IER_RX_FIFO_H: usize = 1 << 0;
pub const UART_RSA_IER_TX_FIFO_H: usize = 1 << 1;
pub const UART_RSA_IER_TX_FIFO_E: usize = 1 << 2;
pub const UART_RSA_IER_RX_TOUT: usize = 1 << 3;
pub const UART_RSA_IER_TIMER: usize = 1 << 4;

pub const UART_RSA_SRR: usize = UART_RSA_BASE + 2;

pub const UART_RSA_SRR_TX_FIFO_NEMP: usize = 1 << 0;
pub const UART_RSA_SRR_TX_FIFO_NHFL: usize = 1 << 1;
pub const UART_RSA_SRR_TX_FIFO_NFUL: usize = 1 << 2;
pub const UART_RSA_SRR_RX_FIFO_NEMP: usize = 1 << 3;
pub const UART_RSA_SRR_RX_FIFO_NHFL: usize = 1 << 4;
pub const UART_RSA_SRR_RX_FIFO_NFUL: usize = 1 << 5;
pub const UART_RSA_SRR_RX_TOUT: usize = 1 << 6;
pub const UART_RSA_SRR_TIMER: usize = 1 << 7;

pub const UART_RSA_FRR: usize = UART_RSA_BASE + 2;

pub const UART_RSA_TIVSR: usize = UART_RSA_BASE + 3;

pub const UART_RSA_TCR: usize = UART_RSA_BASE + 4;

pub const UART_RSA_TCR_SWITCH: usize = 1 << 0;

pub const SERIAL_RSA_BAUD_BASE: usize = 921600;
pub const SERIAL_RSA_BAUD_BASE_LO: usize = SERIAL_RSA_BAUD_BASE / 8;

pub const UART_DA830_PWREMU_MGMT: usize = 12;

pub const UART_DA830_PWREMU_MGMT_FREE: usize = 1 << 0;
pub const UART_DA830_PWREMU_MGMT_URRST: usize = 1 << 13;
pub const UART_DA830_PWREMU_MGMT_UTRST: usize = 1 << 14;

pub const OMAP1_UART1_BASE: usize = 0xfffb0000;
pub const OMAP1_UART2_BASE: usize = 0xfffb0800;
pub const OMAP1_UART3_BASE: usize = 0xfffb9800;
pub const UART_OMAP_MDR1: usize = 0x08;
pub const UART_OMAP_MDR2: usize = 0x09;
pub const UART_OMAP_SCR: usize = 0x10;
pub const UART_OMAP_SSR: usize = 0x11;
pub const UART_OMAP_EBLR: usize = 0x12;
pub const UART_OMAP_OSC_12M_SEL: usize = 0x13;
pub const UART_OMAP_MVER: usize = 0x14;
pub const UART_OMAP_SYSC: usize = 0x15;
pub const UART_OMAP_SYSS: usize = 0x16;
pub const UART_OMAP_WER: usize = 0x17;
pub const UART_OMAP_TX_LVL: usize = 0x1a;

pub const UART_OMAP_MDR1_16X_MODE: usize = 0x00;
pub const UART_OMAP_MDR1_SIR_MODE: usize = 0x01;
pub const UART_OMAP_MDR1_16X_ABAUD_MODE: usize = 0x02;
pub const UART_OMAP_MDR1_13X_MODE: usize = 0x03;
pub const UART_OMAP_MDR1_MIR_MODE: usize = 0x04;
pub const UART_OMAP_MDR1_FIR_MODE: usize = 0x05;
pub const UART_OMAP_MDR1_CIR_MODE: usize = 0x06;
pub const UART_OMAP_MDR1_DISABLE: usize = 0x07;

pub const UART_ALTR_AFR: usize = 0x40;
pub const UART_ALTR_EN_TXFIFO_LW: usize = 0x01;
pub const UART_ALTR_TX_LOW: usize = 0x41;
