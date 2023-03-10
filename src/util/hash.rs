pub fn mix64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

pub fn fold_bytes(data: &[u8], seed: u64) -> u64 {
    let mut h = seed ^ 0x0137;
    for (i, b) in data.iter().enumerate().take(1024) {
        h = h.wrapping_add(*b as u64).wrapping_mul(0x100000001b3);
        h ^= (i as u64).rotate_left(5);
    }
    mix64(h)
}
