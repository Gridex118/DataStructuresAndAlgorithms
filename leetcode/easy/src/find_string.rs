pub fn find(haystack: String, needle: String) -> i32 {
    for i in 0..haystack.len() {
        if needle.len() > haystack[i..].len() {
            break;
        }
        for j in 0..needle.len() {
            if needle.chars().nth(j) == haystack.chars().nth(i + j) {
                if j == needle.len() - 1 {
                    return i as i32;
                }
            } else {
                break;
            }
        }
    }
    -1
}
