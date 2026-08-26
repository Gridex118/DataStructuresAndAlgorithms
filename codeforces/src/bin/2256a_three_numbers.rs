use std::{fmt::Debug, io, str::FromStr};

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn read_t<T>() -> T
where T: FromStr,
      T: Default,
      T::Err: Debug
{
    let input = read();
    match input.trim().parse::<T>() {
        Ok(x) => x,
        Err(_) => T::default()
    }
}

fn read_vec<T>() -> Vec<T>
where T: FromStr,
      T: Default,
      T::Err: Debug
{
    let space_sep_input = read();
    space_sep_input.split_whitespace()
        .map(|s| match s.trim().parse::<T>() {
            Ok(v) => v,
            Err(_) => T::default()
        })
        .collect()
}

/// Choose one of the three current integers and replace it with the sum of the other
/// two current integers. The other two integers remain unchanged
fn min_range(nums: &mut [i32]) -> i32 {
    nums.sort();
    let range = |v: &[i32]| { v[2] - v[0] };
    let old_range = range(&nums);
    nums[2] = nums[0] + nums[1];
    let new_range = range(&nums);
    new_range.min(old_range)
}

fn main() {
    let n: i32 = read_t();
    for _ in 0..n {
        let mut vec = read_vec();
        println!("{}", min_range(&mut vec));
    }
}
