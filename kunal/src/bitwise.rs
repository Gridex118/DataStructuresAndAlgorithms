/// Given two binary strings `a` and `b`, return their sum as a
/// binary string. There are no leading 0s or 1s
pub fn add_binary(a: &str, b: &str) -> String {
    let mut sum = String::new();
    let a = a.as_bytes();
    let b = b.as_bytes();
    let n = a.len();
    let m = b.len();
    let mut carry = 0;
    for i in 0..n.max(m) {
        let digit_a = if i < n {
            a[n - i - 1] - b'0'
        } else { 0 };
        let digit_b = if i < m {
            b[m - i - 1] - b'0'
        } else { 0 };
        sum.push((b'0' + (digit_a ^ digit_b ^ carry)) as char);
        carry = (carry + digit_a + digit_b) / 2;
    }
    if carry == 1 { sum.push('1'); }
    sum.chars().rev().collect()
}

/// Given a non empty array of integers `nums`, every element appears twice
/// except for one. Find that single one
pub fn single_number(nums: &[i32]) -> i32 {
    nums.iter()
        .fold(0, |acc, next| acc ^ next)
}

/// Reverse bits of given 32 bit signed ints
pub fn reverse_bits(n: i32) -> i32 {
    let mut n = n as u32;
    let mut reversed = 0;
    for i in (0..=31).rev() {
        let set_mask = (n & 1) << i;
        reversed |= set_mask;
        n >>= 1;
    }
    reversed as i32
}

/// Given a posivite integer `n`, write a function that returns the number
/// of set bits in its binary representation
pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Given an integer `n`, return an array `ans` of length `n + 1`
/// such that for each `i` (0 <= i <= n), `ans[i]` is the number
/// of 1's in the binary representation of `i`
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut dp = vec![0; n as usize + 1];
    for i in 1..=n {
        if i % 2 == 0 {
            // Doubling a number only adds a zero to the right,
            // the set bits remain the same
            dp[i as usize] = dp[(i / 2) as usize];
        } else {
            dp[i as usize] = dp[(i / 2) as usize] + 1;
        }
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("11", "1"), "100");
        assert_eq!(add_binary("1010", "1011"), "10101");
    }

    #[test]
    fn test_single_number() {
        for (nums, expected) in [
            (vec![2, 2, 1], 1),
            (vec![4, 1, 2, 1, 2], 4),
            (vec![1], 1),
        ] {
            assert_eq!(single_number(&nums), expected);
        }
    }

    #[test]
    fn test_reverse_bits() {
        for (n, expected) in [
            (43261596, 964176192),
            (2147483644, 1073741822),
        ] {
            assert_eq!(reverse_bits(n), expected);
        }
    }

    #[test]
    fn test_hamming_weight() {
        for (n, expected) in [
            (11, 3), (128, 1), (2147483645, 30),
        ] {
            assert_eq!(hamming_weight(n), expected,
                       "Failed for {n}");
        }
    }

    #[test]
    fn test_count_bits() {
        for (n, expected) in [
            (0, vec![0]), (1, vec![0, 1]),
            (2, vec![0, 1, 1]),
            (5, vec![0, 1, 1, 2, 1, 2]),
        ] {
            assert_eq!(count_bits(n), expected,
                       "Failed for {n}");
        }
    }

}
