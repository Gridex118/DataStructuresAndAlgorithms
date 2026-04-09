use std::collections::HashSet;
use std::iter::FromIterator;

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

pub fn missing_number(nums: &Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let nums_set: HashSet<&i32> = HashSet::from_iter(nums);
    for i in 0..(n + 1) {
        if !nums_set.contains(&i) {
            return i;
        }
    }
    n
}

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for x in nums {
        let digits = 1 + (x as f32).log(10.0)
            .floor() as i32;
        if digits % 2 == 0 {
            count += 1;
        }
    }
    count
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter()
        .map(|v| v.iter().sum())
        .max().unwrap()
}

pub fn is_power_of_three(n: i32) -> bool {
    n >= 1 && (1162261467 % n == 0)
}

pub fn is_perfect_square(num: i32) -> bool {
    let num = num as u64;
    let mut start = 0;
    let mut end = num;
    while start <= end {
        let mid = start + (end - start) / 2;
        let mid_sq = mid * mid;
        if mid_sq == num {
            return true;
        } else if mid_sq < num {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    false
}
