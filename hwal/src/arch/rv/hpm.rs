use crate::arch::rv::riscv::register::*;
use crate::write_csr;
const CNT_INH: usize = 0x320;
pub struct PerfMonitor<const N: usize>
where
    [(); N + 1]:,
{
    hpm_start: [u64; N + 1],
}

impl<const N: usize> PerfMonitor<N>
where
    [(); N + 1]:,
{
    pub const fn new() -> Self {
        PerfMonitor {
            hpm_start: [0; N + 1],
        }
    }
    pub fn init(&mut self) {
        self.stop();
        for i in 0..self.hpm_start.len() {
            self.hpm_start[i] = self.hpm_read(i);
        }
    }
    fn hpm_read(&self, idx: usize) -> u64 {
        match idx {
            0 => cycle::read64(),
            1 => instret::read64(),
            2 => 0,
            3 => hpmcounter3::read64(),
            4 => hpmcounter4::read64(),
            5 => hpmcounter5::read64(),
            6 => hpmcounter6::read64(),
            7 => hpmcounter7::read64(),
            8 => hpmcounter8::read64(),
            9 => hpmcounter9::read64(),
            10 => hpmcounter10::read64(),
            11 => hpmcounter11::read64(),
            12 => hpmcounter12::read64(),
            13 => hpmcounter13::read64(),
            14 => hpmcounter14::read64(),
            15 => hpmcounter15::read64(),
            16 => hpmcounter16::read64(),
            17 => hpmcounter17::read64(),
            18 => hpmcounter18::read64(),
            19 => hpmcounter19::read64(),
            20 => hpmcounter20::read64(),
            21 => hpmcounter21::read64(),
            22 => hpmcounter22::read64(),
            23 => hpmcounter23::read64(),
            24 => hpmcounter24::read64(),
            25 => hpmcounter25::read64(),
            26 => hpmcounter26::read64(),
            27 => hpmcounter27::read64(),
            28 => hpmcounter28::read64(),
            29 => hpmcounter29::read64(),
            30 => hpmcounter30::read64(),
            31 => hpmcounter31::read64(),
            _ => panic!("Invalid hpm idx!"),
        }
    }

    pub fn start(&self) {
        write_csr!(CNT_INH, 0);
    }
    pub fn stop(&self) {
        write_csr!(CNT_INH, 0xffffffffusize);
    }
    pub fn cycle(&self) -> u64 {
        cycle::read64() - self.hpm_start[0]
    }
    pub fn insns(&self) -> u64 {
        instret::read64() - self.hpm_start[1]
    }
    pub fn hpm(&self, idx: usize) -> u64 {
        self.hpm_read(idx) - self.hpm_start[idx]
    }
    pub fn hpm_event(&self, hpm: usize, event: usize) {
        match hpm {
            3 => mhpmevent3::write(event),
            4 => mhpmevent4::write(event),
            5 => mhpmevent5::write(event),
            6 => mhpmevent6::write(event),
            7 => mhpmevent7::write(event),
            8 => mhpmevent8::write(event),
            9 => mhpmevent9::write(event),
            10 => mhpmevent10::write(event),
            11 => mhpmevent11::write(event),
            12 => mhpmevent12::write(event),
            13 => mhpmevent13::write(event),
            14 => mhpmevent14::write(event),
            15 => mhpmevent15::write(event),
            16 => mhpmevent16::write(event),
            17 => mhpmevent17::write(event),
            18 => mhpmevent18::write(event),
            19 => mhpmevent19::write(event),
            20 => mhpmevent20::write(event),
            21 => mhpmevent21::write(event),
            22 => mhpmevent22::write(event),
            23 => mhpmevent23::write(event),
            24 => mhpmevent24::write(event),
            25 => mhpmevent25::write(event),
            26 => mhpmevent26::write(event),
            27 => mhpmevent27::write(event),
            28 => mhpmevent28::write(event),
            29 => mhpmevent29::write(event),
            30 => mhpmevent30::write(event),
            31 => mhpmevent31::write(event),
            _ => panic!("Invalid hpm idx!"),
        }
    }
}
