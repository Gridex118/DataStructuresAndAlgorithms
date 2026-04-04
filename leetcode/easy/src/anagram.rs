use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }
    let mut hash_s: HashMap<u8, i32> = HashMap::new();
    for c in s.bytes() {
        if let Some(old_value) = hash_s.insert(c, 1) {
            hash_s.insert(c, old_value + 1);
        }
    }
    for c in t.bytes() {
        if let Some(old_value) = hash_s.insert(c, 1) {
            if old_value == 1 {
                hash_s.remove(&c);
                continue;
            }
            hash_s.insert(c, old_value - 1);
        }
    }
    hash_s.is_empty()
}
