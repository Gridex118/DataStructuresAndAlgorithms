pub fn last_word_length(sentence: String) -> i32 {
    let mut count: i32 = 0;
    for c in sentence.chars().rev() {
        if c.is_whitespace() {
            if count > 0 {
                break;
            }
        } else {
            count += 1;
        }
    }
    count
}
