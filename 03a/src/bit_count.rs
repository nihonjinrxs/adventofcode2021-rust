#[derive(Debug)]
pub struct BitCount {
    pub zeros: u32,
    pub ones:  u32,
}

impl BitCount {
    pub fn new(bit_array: &Vec<u32>) -> Self {
        let mut zeros: u32 = 0;
        let mut ones: u32 = 0;
        for bit in bit_array {
            if      *bit == 0u32 { zeros += 1; }
            else if *bit == 1u32 { ones += 1;  }
        }
        Self {
            zeros: zeros,
            ones:  ones,
        }
    }

    pub fn max_counted_bit(&self) -> u32 {
        if self.zeros > self.ones { 0 }
        else { 1 }
    }

    pub fn min_counted_bit(&self) -> u32 {
        if self.zeros > self.ones { 1 }
        else { 0 }
    }
}
