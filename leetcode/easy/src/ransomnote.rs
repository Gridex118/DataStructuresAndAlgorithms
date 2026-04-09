use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in magazine.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    for c in ransom_note.chars() {
        if let Some(value) = map.get_mut(&c) && *value > 0 {
            *value -= 1;
        } else {
            return false;
        }
    }
    true
}
