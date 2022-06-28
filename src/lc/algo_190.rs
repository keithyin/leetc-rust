
pub fn reverse_bits(mut x: u32) -> u32 {
    let mut result = 0 as u32;
    for i in 0..32 {
        if (x & 1) == 1 {
            result |= 1;
        }
        if i == 31 {
            continue;
        }
        result <<= 1;
        x >>= 1;
    }

    result
}