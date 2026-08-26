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

fn giant_winner(mounts_a: &[i32], mounts_b: &[i32]) -> i32 {
    let lives = |v: &[i32]| {
        let n = v.len() as i32;
        v[0] + n - 1
    };
    if lives(mounts_a) >= lives(mounts_b) {
        1
    } else {
        2
    }
}

#[test]
fn test_winner() {
    assert_eq!(giant_winner(&[ 4, 3, 2, 1 ], &[ 10, 1 ]), 2, "Fail 1");
    assert_eq!(giant_winner(&[ 4, 3, 2, 1 ], &[ 6, 5 ]), 1, "Fail 2");
}

fn main() {
    let n: i32 = read_t();
    for _ in 0..n {
        let _mountains: Vec<i32> = read_vec();
        let mounts_a: Vec<i32> = read_vec();
        let mounts_b: Vec<i32> = read_vec();
        println!("{}", giant_winner(&mounts_a, &mounts_b));
    }
}
