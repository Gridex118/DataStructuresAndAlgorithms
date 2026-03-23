mod pair_sum;
mod palindrome;
mod numerals;
mod lcp;
mod paren;
mod unique_array;
mod rem_element;
mod happy;
mod find_string;
mod insert_pos;
mod last_length;
mod plus_one;
mod add_binary;
mod sqrt;
mod merge_sorted;

use happy::is_happy;
use insert_pos::search_insert;
use last_length::last_word_length;
use sqrt::sqrt;

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
    assert!(k == 5, "Failed to report number of elements after deletion");
    println!("Processed to {:?}", vec3);

    assert!(is_happy(19), "19 reported unhappy");
    assert!(is_happy(7), "7 reported unhappy");
    assert!(!is_happy(2), "2 reported happy");

    let sad_index: i32 = find_string::find(
        String::from("butsad"),
        String::from("sad")
    );
    assert!(sad_index == 3, "Did not index sad correctly");

    let index: i32 = search_insert(vec![ 1, 3, 5, 6 ], 5);
    assert!(index == 2, "1. Wrong insertion index for 5 (got {index} != 2)");
    let index: i32 = search_insert(vec![ 1, 3, 5, 6 ], 2);
    assert!(index == 1, "2. Wrong insertion index for 2 (got {index} != 1)");
    let index: i32 = search_insert(vec![ 1 ], 2);
    assert!(index == 1, "3. Wrong insertion index for 2 (got {index} != 1)");

    let s: String = String::from("Hello World");
    let n: i32 = last_word_length(s);
    assert!(n == 5, "World's length reported as {n}");
    let s: String = String::from("    fly me to the moon  ");
    let n: i32 = last_word_length(s);
    assert!(n == 4, "World's length reported as {n}");
    let s: String = String::from("     ");
    let n: i32 = last_word_length(s);
    assert!(n == 0, "World's length reported as {n}");

    assert_eq!(plus_one::add(vec![ 9, 9, 9 ]), vec![ 1, 0, 0, 0 ], "1. Sum not equal");
    assert_eq!(plus_one::add(vec![ 9, 9, 8 ]), vec![ 9, 9, 9 ], "1. Sum not equal");
    assert_eq!(plus_one::add(vec![ ]), vec![ ], "1. Sum not equal");

    let a: String = String::from("111");
    let b: String = String::from("1");
    let c: String = add_binary::sum(a, b);
    assert_eq!(c, "1000");

    assert_eq!(sqrt(2147395599), 46339, "Reported: sqrt(2147395599) = {}", sqrt(2147395599));
    assert_eq!(sqrt(8), 2, "Reported: sqrt(8) = {}", sqrt(8));
    assert_eq!(sqrt(0), 0, "Reported: sqrt(0) = {}", sqrt(0));
    assert_eq!(sqrt(3), 1, "Reported: sqrt(3) = {}", sqrt(3));

    let mut nums1: Vec<i32> = vec![0];
    merge_sorted::merge(&mut nums1, 0, &mut [1], 1);
    println!("{:?}", nums1);
}
