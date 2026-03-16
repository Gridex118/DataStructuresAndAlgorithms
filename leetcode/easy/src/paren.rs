fn get_closing(opening: char) -> Option<char> {
    match opening {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _ => None
    }
}

pub fn is_valid(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        if get_closing(c).is_some() {
            stack.push(c);
        } else if let Some(popped) = stack.pop() {
            if Some(c) != get_closing(popped) {
                return false;
            }
        } else {
            return false;
        }
    }
    stack.is_empty()
}
