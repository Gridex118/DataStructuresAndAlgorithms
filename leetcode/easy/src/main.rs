mod pair_sum;
mod palindrome;
mod numerals;
mod lcp;

fn main() {
    let vec1: Vec<i32> = vec![ 3, 3 ];
    println!("The pair is {:?}", pair_sum::sum(&vec1, 6));

    let num1: i32 = 121;
    println!("{num1} is a palindrome: {}", palindrome::is_palindrome(num1));

    let roman1: String = String::from("MCMXCIV");
    println!("{roman1} as integer: {}", numerals::roman_to_int(&roman1));

    let strs: Vec<String> = vec![ "flower".to_string(), "flow".into(), "flight".into() ];
    println!("Common prefix: {}", lcp::get_prefix(strs));
}
