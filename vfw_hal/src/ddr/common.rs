pub trait Ddr {
    fn size(&self) -> u64 {
        self.layout().total_size()
    }
    fn base(&self) -> u64;
    fn layout(&self) -> DdrLayout;
}

pub trait DdrDriver: Ddr {
    fn init_ddr(&self);
    fn init_clk(&self);
}

pub enum DdrSpeed {
    Spd1600,
    Spd800,
    Spd533,
    Spd500,
    Spd400,
    Spd266,
}

impl DdrSpeed {
    pub const fn t_ck(&self) -> f32 {
        match self {
            DdrSpeed::Spd1600 => 0.625,
            DdrSpeed::Spd800 => 1.25,
            DdrSpeed::Spd533 => 1.876,
            DdrSpeed::Spd500 => 2.0,
            DdrSpeed::Spd400 => 2.5,
            DdrSpeed::Spd266 => 3.759,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(usize)]
pub enum DdrRankSize {
    Rank128Mbyte = 1,
    Rank256Mbyte,
    Rank384Mbyte,
    Rank512Mbyte,
    Rank768Mbyte,
    Rank1024Mbyte,
    Rank1536Mbyte,
    Rank2048Mbyte,
}

impl DdrRankSize {
    pub const fn size(&self) -> usize {
        match self {
            DdrRankSize::Rank128Mbyte => 0x8000000,
            DdrRankSize::Rank256Mbyte => 0x10000000,
            DdrRankSize::Rank384Mbyte => 0x18000000,
            DdrRankSize::Rank512Mbyte => 0x20000000,
            DdrRankSize::Rank768Mbyte => 0x30000000,
            DdrRankSize::Rank1024Mbyte => 0x40000000,
            DdrRankSize::Rank1536Mbyte => 0x60000000,
            DdrRankSize::Rank2048Mbyte => 0x80000000,
        }
    }
    pub const fn width(&self) -> usize {
        self.size().next_power_of_two().trailing_zeros() as usize
    }
}

impl core::convert::From<usize> for DdrRankSize {
    fn from(v: usize) -> Self {
        match v {
            1 => DdrRankSize::Rank128Mbyte,
            2 => DdrRankSize::Rank256Mbyte,
            3 => DdrRankSize::Rank384Mbyte,
            4 => DdrRankSize::Rank512Mbyte,
            5 => DdrRankSize::Rank768Mbyte,
            6 => DdrRankSize::Rank1024Mbyte,
            7 => DdrRankSize::Rank1536Mbyte,
            8 => DdrRankSize::Rank2048Mbyte,
            _ => DdrRankSize::Rank1024Mbyte,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(usize)]
pub enum DdrRankNum {
    Rank1 = 1,
    Rank2 = 2,
    Rank4 = 4,
}

impl core::convert::From<usize> for DdrRankNum {
    fn from(v: usize) -> Self {
        match v {
            1 => DdrRankNum::Rank1,
            2 => DdrRankNum::Rank2,
            4 => DdrRankNum::Rank4,
            _ => DdrRankNum::Rank1,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(usize)]
pub enum DdrBl {
    Bl2 = 2,
    Bl4 = 4,
    Bl8 = 8,
    Bl16 = 16,
}

impl core::convert::From<usize> for DdrBl {
    fn from(v: usize) -> Self {
        match v {
            2 => DdrBl::Bl2,
            4 => DdrBl::Bl4,
            8 => DdrBl::Bl8,
            16 => DdrBl::Bl16,
            _ => DdrBl::Bl16,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(C)]
pub struct DdrRankLayout {
    pub col_width: usize,
    pub row_width: usize,
    pub bank_width: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, PartialOrd)]
#[repr(C)]
pub struct DdrLayout {
    pub ranks: Option<DdrRankNum>,
    pub rank_size: DdrRankSize,
    pub bus_width: usize, //in bytes, per channel
    pub chs: usize,
    pub bl: DdrBl,
    pub cols: usize,
    pub rows: usize,
    pub banks: usize,
}

impl DdrLayout {
    const fn get_width(&self, v: usize) -> usize {
        v.next_power_of_two().trailing_zeros() as usize
    }
    pub const fn data_width(&self) -> usize {
        self.get_width(self.bus_width * self.chs)
    }
    pub const fn col_width(&self) -> usize {
        self.get_width(self.cols) + self.get_width(self.bl as usize)
    }
    pub const fn row_width(&self) -> usize {
        self.get_width(self.rows)
    }
    pub const fn bank_width(&self) -> usize {
        self.get_width(self.banks)
    }
    pub fn rank_width(&self) -> Option<usize> {
        self.ranks.map(|r| self.get_width(r as usize))
    }
    pub fn total_size(&self) -> u64 {
        (self.ranks.unwrap_or(DdrRankNum::Rank1) as usize as u64)
            * (self.rank_size.size() as u64)
            * (self.chs as u64)
    }
    pub const fn ddr_1bank_width(&self) -> usize {
        self.col_width() + self.row_width()
    }
    pub const fn ddr_1rank_width(&self) -> usize {
        self.ddr_1bank_width() + self.bank_width()
    }
    pub fn ddr_width(&self) -> usize {
        self.ddr_1rank_width() + self.rank_width().unwrap_or(0)
    }
    pub fn total_width(&self) -> usize {
        self.ddr_width() + self.data_width()
    }
}
