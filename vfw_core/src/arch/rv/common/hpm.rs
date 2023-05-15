use crate::write_csr;
use riscv::register::*;
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
            0 => mcycle::read64(),
            1 => minstret::read64(),
            2 => 0,
            3 => mhpmcounter3::read64(),
            4 => mhpmcounter4::read64(),
            5 => mhpmcounter5::read64(),
            6 => mhpmcounter6::read64(),
            7 => mhpmcounter7::read64(),
            8 => mhpmcounter8::read64(),
            9 => mhpmcounter9::read64(),
            10 => mhpmcounter10::read64(),
            11 => mhpmcounter11::read64(),
            12 => mhpmcounter12::read64(),
            13 => mhpmcounter13::read64(),
            14 => mhpmcounter14::read64(),
            15 => mhpmcounter15::read64(),
            16 => mhpmcounter16::read64(),
            17 => mhpmcounter17::read64(),
            18 => mhpmcounter18::read64(),
            19 => mhpmcounter19::read64(),
            20 => mhpmcounter20::read64(),
            21 => mhpmcounter21::read64(),
            22 => mhpmcounter22::read64(),
            23 => mhpmcounter23::read64(),
            24 => mhpmcounter24::read64(),
            25 => mhpmcounter25::read64(),
            26 => mhpmcounter26::read64(),
            27 => mhpmcounter27::read64(),
            28 => mhpmcounter28::read64(),
            29 => mhpmcounter29::read64(),
            30 => mhpmcounter30::read64(),
            31 => mhpmcounter31::read64(),
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
        mcycle::read64() - self.hpm_start[0]
    }
    pub fn insns(&self) -> u64 {
        minstret::read64() - self.hpm_start[1]
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
