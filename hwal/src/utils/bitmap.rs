const MAP_ENTRY_BITS: usize = usize::BITS as usize;
pub const fn map_size(size: usize) -> usize {
    (size + MAP_ENTRY_BITS - 1) / MAP_ENTRY_BITS
}

#[derive(Copy, Clone)]
pub struct BitMap<const N: usize>
where
    [(); map_size(N)]: ,
{
    map: [usize; map_size(N)],
}

impl<const N: usize> BitMap<N>
where
    [(); map_size(N)]: ,
{
    pub const fn new() -> BitMap<N> {
        BitMap {
            map: [0; map_size(N)],
        }
    }

    fn find_bit(&self, set: bool, from: usize, to: usize) -> Option<usize> {
        let from_i = from / MAP_ENTRY_BITS;
        let to_i = (to + MAP_ENTRY_BITS - 2) / MAP_ENTRY_BITS;
        for i in from_i..=to_i {
            if self.map[i] != if set { 0 } else { !0 } {
                let pos = i * MAP_ENTRY_BITS
                    + if set {
                        self.map[i].trailing_zeros()
                    } else {
                        self.map[i].trailing_ones()
                    } as usize;
                if pos >= to {
                    return None;
                } else if pos >= from {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn bit_pos(&self, pos: usize) -> (usize, usize) {
        assert!(pos < N);
        (pos / MAP_ENTRY_BITS, pos % MAP_ENTRY_BITS)
    }
    pub fn find_1st0(&self) -> Option<usize> {
        self.find_bit(false, 0, N)
    }

    pub fn find_1st1(&self) -> Option<usize> {
        self.find_bit(true, 0, N)
    }

    pub fn find_1st0_range(&self, from: usize, to: usize) -> Option<usize> {
        self.find_bit(false, from, to)
    }

    pub fn find_1st1_range(&self, from: usize, to: usize) -> Option<usize> {
        self.find_bit(true, from, to)
    }

    pub fn set_bit(&mut self, pos: usize) {
        let (i, r) = self.bit_pos(pos);
        self.map[i] |= 1 << r;
    }

    pub fn clr_bit(&mut self, pos: usize) {
        let (i, r) = self.bit_pos(pos);
        self.map[i] &= !(1 << r);
    }

    pub fn bit(&self, pos: usize) -> usize {
        let (i, r) = self.bit_pos(pos);
        (self.map[i] >> r) & 1
    }

    fn bits_pos(&self, pos: usize, width: usize) -> (usize, usize, usize, usize) {
        assert!(width <= MAP_ENTRY_BITS && width > 0);
        assert!(pos + width <= N);
        let msb = pos + width - 1;
        let (i, r) = self.bit_pos(pos);
        let (i_m, r_m) = self.bit_pos(msb);
        (i, r, i_m, r_m)
    }

    pub fn set_bits(&mut self, pos: usize, width: usize) {
        let (i, r, i_m, r_m) = self.bits_pos(pos, width);
        if i == i_m {
            self.map[i] |= (((1 << (width - 1)) - 1) | 1 << (width - 1)) << r;
        } else {
            self.map[i] |= ((1 << (MAP_ENTRY_BITS - r)) - 1) << r;
            self.map[i_m] |= ((1 << r_m) - 1) | (1 << r_m);
        }
    }

    pub fn clr_bits(&mut self, pos: usize, width: usize) {
        let (i, r, i_m, r_m) = self.bits_pos(pos, width);
        if i == i_m {
            self.map[i] &= !((((1 << (width - 1)) - 1) | 1 << (width - 1)) << r);
        } else {
            self.map[i] &= !(((1 << (MAP_ENTRY_BITS - r)) - 1) << r);
            self.map[i_m] &= !(((1 << r_m) - 1) | (1 << r_m));
        }
    }

    pub fn bits(&self, pos: usize, width: usize) -> usize {
        let (i, r, i_m, r_m) = self.bits_pos(pos, width);
        if i == i_m {
            (self.map[i] >> r) & (((1 << (width - 1)) - 1) | 1 << (width - 1))
        } else {
            (self.map[i] >> r) | ((self.map[i_m] & ((1 << r_m) - 1)) << (MAP_ENTRY_BITS - r))
        }
    }
}
