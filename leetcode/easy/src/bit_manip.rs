pub fn reverse_bits(mut v: u32) -> u32 {
    let mut r = 0;
    for i in 0..32 {
        r |= (v & 1) << (31 - i);
        v >>= 1;
    }
    r
}

pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n & 0b1;
        n >>= 1;
    }
    count
}

pub fn find_complement(num: i32) -> i32 {
    let i_msb = if num == 0 { 1 } else { num.ilog2() + 1 };
    num ^ ((0b1u32 << i_msb) - 1) as i32
}
