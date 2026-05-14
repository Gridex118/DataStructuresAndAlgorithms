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

fn _find_words(words: Vec<String>) -> Vec<String> {
    let row_sets: [HashSet<char>; 3] = [
        HashSet::from_iter("qwertyuiop".chars()),
        HashSet::from_iter("asdfghjkl".chars()),
        HashSet::from_iter("zxcvbnm".chars())
    ];
    words.into_iter()
        .filter(|word| {
            let set = HashSet::from_iter(word.to_ascii_lowercase().chars());
            row_sets.iter().any(|row_set| row_set.is_superset(&set))
        })
        .collect()
}

fn _base7(num: i32) -> String {
    match num {
        0 => "0".to_string(),
        _ => {
            let mut converted = String::new();
            let negate = num < 0;
            let mut num = num.abs();
            while num > 0 {
                converted.push((b'0' + (num % 7) as u8) as char);
                num /= 7;
            }
            if negate { converted.push('-'); }
            converted.chars().rev()
                .collect() 
        }
    }
}

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let m = list1.len();
    let n = list2.len();
    let list_ref = if m <= n { &list1 } else { &list2 };
    let other_list_ref = if m <= n { &list2 } else { &list1 };
    let mut other_list_set = HashMap::with_capacity(other_list_ref.len());
    for (i, string) in other_list_ref.iter().enumerate() {
        other_list_set.insert(string, i);
    }
    let mut index_sums = vec![m + n; list_ref.len()];
    let mut min_index_sum = m + n;
    for (i, string) in list_ref.iter().enumerate() {
        if let Some(j) = other_list_set.get(string) {
            let index_sum = i + j;
            index_sums[i] = index_sum;
            min_index_sum = min_index_sum.min(index_sum);
        }
    }
    let mut answer = Vec::with_capacity(list_ref.len());
    for (i, &sum) in index_sums.iter().enumerate() {
        if sum == min_index_sum {
            answer.push(list_ref[i].to_string());
        }
    }
    answer
}
