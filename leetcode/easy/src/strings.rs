use std::collections::HashMap;

pub fn first_uniq_chars(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    for (i, c) in s.chars().enumerate() {
        if map[&c] == 1 {
            return i as i32;
        }
    }
    -1
}

pub fn _find_the_difference(s: String, t: String) -> char {
    s.bytes().chain(t.bytes())
        .reduce(|acc, next| acc ^ next).unwrap()
        as char
}

pub fn count_segments(s: String) -> i32 {
    let mut count = 0;
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    for i in 0..n {
        if s[i] != ' ' && (i == 0 || s[i - 1] == ' ') {
            count += 1;
        }
    }
    count
}
