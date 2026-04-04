use std::collections::HashMap;

pub fn are_isomorphic(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    if s.len() != t.len() { return false; }
    let mut s_to_t: HashMap<char, char> = HashMap::new();
    let mut t_to_s: HashMap<char, char> = HashMap::new();
    for (i, &c) in s.iter().enumerate() {
        if let Some(old) = s_to_t.insert(c, t[i])
            && old != t[i] {
                return false;
            }
    }
    for (i, &c) in t.iter().enumerate() {
        if let Some(old) = t_to_s.insert(c, s[i])
            && old != s[i] {
                return false;
            }
    }
    true
}
