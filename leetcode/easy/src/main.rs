mod pair_sum;
mod palindrome;
mod numerals;
mod lcp;
mod paren;
mod unique_array;
mod rem_element;

fn main() {
    let vec1: Vec<i32> = vec![ 3, 3 ];
    println!("The pair is {:?}", pair_sum::sum(&vec1, 6));

    let num1: i32 = 121;
    println!("{num1} is a palindrome: {}", palindrome::is_palindrome(num1));

    let roman1: String = String::from("MCMXCIV");
    println!("{roman1} as integer: {}", numerals::roman_to_int(&roman1));

    let strs: Vec<String> = vec![ "flower".to_string(), "flow".into(), "flight".into() ];
    println!("Common prefix: {}", lcp::get_prefix(strs));

    assert!(paren::is_valid("()[]{}"), "()[]{{}} reported invalid");
    assert!(paren::is_valid("([])"), "([]) reported invalid");
    assert!(!paren::is_valid("(][)"), "(][) reported valid");
    assert!(!paren::is_valid("([)]"), "([)] reported valid");

    let mut vec2: Vec<i32> = vec![ 0, 0, 1, 1, 1, 2, 2, 3, 3, 4 ];
    println!("Vector: {:?}", vec2);
    let k: i32 = unique_array::remove_duplicates(&mut vec2);
    println!("k = {k}");
    assert!(k == 5, "Failed to report number of unique elements");
    println!("Processed to {:?}", vec2);

    let mut vec3: Vec<i32> = vec![ 0, 1, 2, 2, 3, 0, 4, 2 ];
    println!("Vector: {:?}", vec3);
    let k: i32 = rem_element::remove_element(&mut vec3, 2);
    println!("k = {k}");
    // assert!(k == 5, "Failed to report number of elements after deletion");
    println!("Processed to {:?}", vec3);
}
