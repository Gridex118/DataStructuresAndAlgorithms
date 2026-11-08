use std::collections::HashSet;

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

/// A binary watch has 4 LEDs on the top to represent the hours (0-11), and
/// 6 LEDs on the bottom to represent the minutes (0-59). Each LED
/// represents a zero or one, with the least significant bit on the right.
///
/// Given an integer `turned_on` which represents the number of LEDs that are
/// currently on (ignoring the PM), *return all possible times the watch
/// could represent*.
pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let set_bits_map = count_bits(59);
    let mut result = Vec::new();
    for h in 0..12 {
        let set_bits_in_hour = set_bits_map[h];
        for min in 0..60 {
            if set_bits_map[min] + set_bits_in_hour == turned_on {
                result.push(format!("{}:{:02}", h, min));
            }
        }
    }
    result
}

/// Given two integers `x` and `y`, return the number of positions at which
/// the corresponding bits are different
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    // = Number of set bits in x XOR y
    (x ^ y).count_ones() as i32
}

/// The complement of an integer is the integer you get when you flip all the
/// 0s to 1s and all the 1s to 0s in its binary representation
///
/// Given an integer `num`, return its complement (note that the word
/// length is taken as the position of the most significant set bit)
pub fn find_complement(num: i32) -> i32 {
    let pos_leftmost_set_bit = num.ilog2() + 1;
    // Mask to select the same number of non padding digits as in `num`
    let and_mask = (1 << pos_leftmost_set_bit) - 1;
    !num & and_mask
}

/// You have a set of integers `s`, which originally contains the numbers from
/// 1 to n. Unfortunately, due to some error, one of the numbers in `s` got
/// duplicated to another number in the set, which results in repetition
/// of one number and loss of another number.
///
/// You are given n integer array `nums` representing the data status of this
/// set after the error. Find the number that occurs twice and the number that
/// is missing
pub fn find_error_nums(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let x_xor_y = (1..=n).fold(0, |acc, i| {
        acc ^ (i as i32) ^ nums[i - 1]
    });
    let rightmost_mask = x_xor_y & (-x_xor_y);
    let (mut x, mut y) = (0, 0);
    for i in 1..=n {
        if i as i32 & rightmost_mask == 0 {
            x ^= i as i32;
        } else {
            y ^= i as i32;
        }
        if nums[i - 1] & rightmost_mask == 0 {
            x ^= nums[i - 1];
        } else {
            y ^= nums[i - 1];
        }
    }
    for &num in nums {
        if num == x {
            return vec![x, y];
        }
    }
    vec![y, x]
}

/// Given a positive integer, check whether it has alternating bits: namely,
/// if two adjacent bits will always have different values.
pub fn has_alternating_bits(n: i32) -> bool {
    let shifted_xor = n ^ (n >> 1);
    shifted_xor & (shifted_xor + 1) == 0
}

/// Given two integers `left` and `right`, return the count of numbers in the
/// inclusive range [left, right] having a prime number of set bits in their
/// binary representation
pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let primes = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19]);
    let mut count = 0;
    for num in left..=right {
        if primes.contains(&(num.count_ones())) {
            count += 1;
        }
    }
    count
}

/// Given a positive integer `n`, find and return the longest distance
/// between any two adjacent 1's in the binary representation of `n`. If
/// there are no two adjacent 1's return 0.
pub fn binary_gap(mut n: i32) -> i32 {
    let mut max_gap = 0;
    let mut current_pos = 0;
    let mut last_pos = 32;      // i32, indices 0..=31
    while n > 0 {
        if n & 1 == 1 {
            max_gap = max_gap.max(current_pos - last_pos);
            last_pos = current_pos;
        }
        current_pos += 1;
        n >>= 1;
    }
    max_gap
}

/// Given an integer `num`, return number of steps to reduce it to zero.
///
/// In one step, if the current number is even, you have to divide it
/// by 2, otherwise, you have to subtract 1 from it.
pub fn number_of_steps(mut num: i32) -> i32 {
    let mut steps = 0;
    while num > 0 {
        steps += 1;
        num = if num & 1 == 1 {
            num - 1
        } else {
            num >> 1
        };
    }
    steps
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

    #[test]
    fn test_read_binary_watch() {
        for (turned_on, expected) in [
            (1, vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32",
                "1:00", "2:00", "4:00", "8:00"
            ]),
            (9, vec![]),
        ] {
            assert_eq!(read_binary_watch(turned_on), expected,
                       "Failed for {turned_on}")
        }
    }

    #[test]
    fn test_hamming_distance() {
        for (x, y, expected) in [
            (1, 4, 2), (3, 1, 1),
        ] {
            assert_eq!(hamming_distance(x, y), expected,
                       "Failed for {x}, {y}");
        }
    }

    #[test]
    fn test_find_complement() {
        for (num, expected) in [
            (5, 2), (1, 0),
        ] {
            assert_eq!(find_complement(num), expected,
                       "Failed for {num}");
        }
    }

    #[test]
    fn test_find_error_nums() {
        for (nums, expected) in [
            (vec![1, 2, 2, 4], vec![2, 3]),
            (vec![1, 1], vec![1, 2])
        ] {
            assert_eq!(find_error_nums(&nums), expected,
                       "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_has_alternating_bits() {
        for (n, expected) in [
            (5, true), (7, false), (11, false),
        ] {
            assert_eq!(has_alternating_bits(n), expected,
                       "Failed for {n}");
        }
    }

    #[test]
    fn test_count_prime_set_bits() {
        for (left, right, expected) in [
            (6, 10, 4), (10, 15, 5)
        ] {
            assert_eq!(count_prime_set_bits(left, right), expected,
                       "Failed for {left}, {right}");
        }
    }

    #[test]
    fn test_binary_gap() {
        for (n, expected) in [
            (22, 2), (8, 0), (5, 2),
        ] {
            assert_eq!(binary_gap(n), expected,
                       "Failed for {n}");
        }
    }

    #[test]
    fn test_number_of_steps() {
        for (num, expected) in [
            (14, 6), (8, 4), (123, 12),
        ] {
            assert_eq!(number_of_steps(num), expected,
                       "Failed for {num}");
        }
    }

}
