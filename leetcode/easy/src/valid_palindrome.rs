pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() { return true; }
    let s = s.as_bytes();
    let mut l: usize = 0;
    let mut r: usize = s.len() - 1;
    while l < r {
        if !(s[l] as char).is_alphanumeric() {
            l += 1;
            continue;
        }
        if !(s[r] as char).is_alphanumeric() {
            r -= 1;
            continue;
        }
        if !s[l].eq_ignore_ascii_case(&s[r]) {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
