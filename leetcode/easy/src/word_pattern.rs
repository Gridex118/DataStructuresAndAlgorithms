use std::collections::{HashMap, BTreeSet};

pub fn word_pattern(pattern: String, s: String) -> bool {
    let s = s.split(" ").collect::<Vec<&str>>();
    let pattern = pattern.as_bytes();
    if s.len() != pattern.len() { return false; }
    let mut pattern_map: HashMap<u8, &str> = HashMap::new();
    for (i, word) in s.iter().enumerate() {
        if let Some(old_value) = pattern_map.insert(pattern[i], word)
            && &old_value != word {
                return false;
            }
    }
    pattern_map.values().collect::<BTreeSet<_>>().len() == pattern_map.values().len()
}
