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
