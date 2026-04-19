use std::collections::{HashMap, HashSet};

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

pub fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    let mut i = n / 2;
    while i > 0 {
        if n.is_multiple_of(i) {
            let mut tmp = String::new();
            let sub_string = &s[0..i];
            for _ in 0..(n / i) {
                tmp.push_str(sub_string);
            }
            if tmp == s {
                return true;
            }
        }
        i -= 1;
    }
    false
}

pub fn licence_key_formatting(s: String, k: i32) -> String {
    let unformatted: String = s.chars()
        .filter(|&c| c != '-')
        .collect();
    let n = unformatted.len() as i32;
    let mut i = 0;
    let mut formatted = String::new();
    for c in unformatted.chars().rev() {
        formatted.push(c.to_ascii_uppercase());
        i += 1;
        if (i % k == 0) && (i != n) {
            formatted.push('-');
        }
    }
    formatted.chars().rev()
        .collect()
}

fn _check_if_pangram(sentence: String) -> bool {
    let mut used = [false; 26];
    for b in sentence.bytes() {
        used[(b - b'a') as usize] = true;
    }
    used.iter().all(|&b| b)
}
