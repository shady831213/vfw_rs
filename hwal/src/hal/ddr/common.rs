pub trait Ddr {
    fn size(&self) -> usize;
    fn base(&self) -> usize; 
}


pub trait DdrDriver:Ddr {
    fn init_ddr(&self);
    fn init_clk(&self);
}

pub enum DdrSpeed {
    Spd1600,
    Spd533,
    Spd500,
    Spd400,
    Spd266,
}

impl DdrSpeed {
    pub const fn t_ck(&self) -> f32 {
        match self {
            DdrSpeed::Spd1600 => 0.625,
            DdrSpeed::Spd533 => 1.876,
            DdrSpeed::Spd500 => 2.0,
            DdrSpeed::Spd400 => 2.5,
            DdrSpeed::Spd266 => 3.759,
        }
    }
}