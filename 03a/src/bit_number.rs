fn fold_accum(acc: u32, bit: &u32) -> u32 {
  ((acc << 1) + *bit) as u32
}

pub fn number_from_bit_array(bits: &Vec<u32>) -> u32 {
  bits.iter().fold(0, fold_accum)
}
