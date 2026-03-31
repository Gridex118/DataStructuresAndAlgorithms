pub fn add_digits(num: i32) -> i32 {
    if num == 0 {
        0
    } else {
        (num - 1) % 9 + 1
    }
}

pub fn is_ugly(mut num: i32) -> bool {
    if num < 1 {
        false
    } else {
        for prime in [2, 3, 5] {
            while num % prime == 0 {
                num /= prime
            }
        }
        num == 1
    }
}
