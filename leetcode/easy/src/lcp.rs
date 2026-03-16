pub fn get_prefix(strs: Vec<String>) -> String {
    let mut prefix: String = String::new();
    let Some(string) = strs.iter().min_by_key(|&s| s.len()) else {
        panic!("Something weird happened");
    };
    for (i, c) in string.chars().enumerate() {
        for other_str in &strs {
            if other_str.chars().nth(i) != Some(c) {
                return prefix;
            }
        }
        prefix.push(c);
    }
    prefix
}
