// fn palindrome(x: i32) -> i32 {
//     let mut num: i32 = x;
//     let mut reverse: i32 = 0;
//     while num > 0 {
//         reverse = (reverse * 10) + (num % 10);
//         num /= 10;
//     }
//     reverse
// }

// pub fn is_palindrome(x: i32) -> bool {
//     if x < 0 {
//         false
//     } else {
//         x == palindrome(x)
//     }
// }

pub fn is_palindrome(x: i32) -> bool {
    let x_str: String = x.to_string();
    let n: usize = x_str.len();
    for i in 0..(n / 2) {
        if x_str.as_bytes()[i] != x_str.as_bytes()[n - 1 - i] {
            return false;
        }
    }
    true
}
