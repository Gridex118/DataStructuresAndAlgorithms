pub fn reverse_vowels(s: String) -> String {
    if s.is_empty() { return String::new(); }
    let vowels = String::from("aeiouAEIOU");
    let mut s = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        while left < right && !vowels.contains(s[left]) {
            left += 1;
        }
        while left < right && !vowels.contains(s[right]) {
            right -= 1;
        }
        if left >= right { break; }
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
    s.iter().collect()
}
