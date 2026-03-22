pub fn sum(a: String, b: String) -> String {
    let mut c: String = String::from("");
    let mut i: i32 = a.len() as i32 - 1;
    let mut j: i32 = b.len() as i32 - 1;
    let mut carry: u8 = 0;
    while i >= 0 || j >= 0 {
        let a_num: u8 = if i >= 0 { a.as_bytes()[i as usize] - b'0' } else { 0 };
        let b_num: u8 = if j >= 0 { b.as_bytes()[j as usize] - b'0' } else { 0 };
        carry += a_num + b_num;
        c.push((b'0' + carry % 2) as char);
        i -= 1;
        j -= 1;
        carry /= 2;
    }
    if carry == 1 {
        c.push('1');
    }
    c.chars().rev().collect::<String>()
}
