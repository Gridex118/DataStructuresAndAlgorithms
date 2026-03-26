fn alphabet(offset: i32) -> char {
    (b'A' + offset as u8) as char
}

pub fn convert_to_title(mut column_number: i32) -> String {
    let mut title = String::new();
    while column_number > 0 {
        column_number -= 1;
        let rem = column_number % 26;
        title.push(alphabet(rem));
        column_number /= 26;
    }
    title.chars().rev().collect()
}

pub fn title_to_col(column_title: String) -> i32 {
    column_title.as_bytes().iter()
        .fold(0, |acc, c| acc * 26 + (c - b'A' + 1) as i32)
}
