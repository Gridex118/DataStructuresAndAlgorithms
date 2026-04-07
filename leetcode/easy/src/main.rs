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
mod valid_palindrome;
mod single_number;
mod excel;
mod bit_manip;
mod summary_ranges;
mod numbers;
mod move_zeros;
mod contains_duplicate;
mod anagram;
mod word_pattern;
mod isomorphism;
mod reverse_string;
mod arrays;

use happy::is_happy;
use insert_pos::search_insert;
use last_length::last_word_length;
use sqrt::sqrt;
use valid_palindrome::is_palindrome;
use single_number::single_number;
use bit_manip::*;
use summary_ranges::summary_ranges;
use numbers::*;
use move_zeros::move_zeros;
use contains_duplicate::*;
use anagram::is_anagram;
use word_pattern::word_pattern;
use isomorphism::are_isomorphic;
use reverse_string::reverse_vowels;

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

    assert!(is_palindrome(String::from("A man, a plan, a canal: Panama")),
            "Reported 'a man...' is not a palindrome");
    assert!(!is_palindrome(String::from("race a car")),
            "Reported 'race...' is a palindrome");
    assert!(is_palindrome(String::from(" ")),
            "Reported empty string is not a palindrome");

    let vec: Vec<i32> = vec![ 2, 2, 1 ];
    let ans: i32 = single_number(vec);
    assert_eq!(ans, 1, "Reported {ans} instead of 1");
    let vec: Vec<i32> = vec![ 4, 1, 2, 1, 2 ];
    let ans: i32 = single_number(vec);
    assert_eq!(ans, 4, "Reported {ans} instead of 4");

    for (col, expected_title) in [
        (1, "A"), (28, "AB"), (52, "AZ"),
        (701, "ZY"), (703, "AAA"),
    ] {
        let title: String = excel::convert_to_title(col);
        assert_eq!(title, expected_title, "Reported '{title}' for column {col}");
        let result_col: i32 = excel::title_to_col(String::from(expected_title));
        assert_eq!(result_col, col, "Reported '{result_col}' for title {expected_title}");
    }

    for (original, reversed) in [
        (43261596, 964176192), (2147483644, 1073741822)
    ] {
        let result = reverse_bits(original);
        assert_eq!(result, reversed, "Reverse of {original}, reported as {result}");
    }

    for (n, count) in [        
    ] {
        let result = hamming_weight(n);
        assert_eq!(result, count, "Count for {n} reported as {result}");
    }

    assert_eq!(summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
               vec!["0", "2->4", "6", "8->9"]);
    assert_eq!(summary_ranges(vec![] as Vec<i32>), vec![] as Vec<String>);
    assert_eq!(summary_ranges(vec![0]), vec!["0"]);

    for (num, root) in [
        (38, 2), (42, 6), (1, 1), (0, 0)
    ] {
        let result = add_digits(num);
        assert_eq!(result, root, "add_digits({num}) gave {result}");
    }
    for (num, ugliness) in [
        (0, false), (-1, false), (1, true),
        (35, false), (6, true)
    ] {
        let result = is_ugly(num);
        assert_eq!(result, ugliness, "is_ugly({num}) gave {result}");
    }
    for (mut nums, sorted) in [
        (vec![1], vec![1]),
        (vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]),
        (vec![0, 2, 3, 0, 0, 4, 5, 7, 7], vec![2, 3, 4, 5, 7, 7, 0, 0, 0])
    ] {
        move_zeros(&mut nums);
        assert_eq!(nums, sorted);
    }

    for (nums, missing) in [
        (vec![3, 0, 1], 2),
        (vec![0, 1], 2),
        (vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8),
    ] {
        let result = missing_number(&nums);
        assert_eq!(result, missing, "Reported {result} as missing in {:?}", nums);
    }

    for (nums, truth) in [
        (vec![1, 2, 3, 1], true),
        (vec![1, 2, 3, 4], false),
    ] {
        let result = contains_duplicate(nums);
        assert_eq!(result, truth);
    }

    for (nums, count) in [
        (vec![12, 345, 2, 6, 7896], 2),
        (vec![555, 901, 482, 1771], 1),
    ] {
        let result = find_numbers(nums);
        assert_eq!(result, count);
    }

    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![0, 1]]), 6);

    for (s, t, truth) in [
        (String::from("anagram"), String::from("nagaram"), true),
        (String::from("rat"), String::from("cat"), false),
    ] {
        assert_eq!(is_anagram(s, t), truth);
    }

    for (pattern, s, truth) in [
        ("abba", "dog cat cat dog", true),
        ("aba", "cat frog pig", false),
        ("aba", "dog cat cat dog", false),
        ("abba", "dog dog dog dog", false),
    ] {
        assert_eq!(word_pattern(String::from(pattern), String::from(s)), truth, "{pattern}:{s}");
    }
    for (s, t, truth) in [
        ("egg", "add", true),
        ("f11", "b23", false),
        ("paper", "title", true),
        ("xyz", "1234", false),
        ("badc", "baba", false),
    ] {
        assert_eq!(are_isomorphic(String::from(s), String::from(t)), truth, "{s}:{t}");
    }

    for (nums, k, truth) in [
        (vec![1, 2, 3, 1], 3, true),
        (vec![1, 0, 1, 1], 1, true),
        (vec![1, 2, 3, 1, 2, 3], 2, false),
    ] {
        assert_eq!(contains_nearby_duplicate(nums.to_vec(), k), truth, "{:?}", nums);
    }

    let powers_in_range = (0..244)
        .filter(|&x| is_power_of_three(x))
        .collect::<Vec<i32>>();
    assert_eq!(powers_in_range, vec![1, 3, 9, 27, 81, 243]);

    for (s, expect) in [
        ("leetcode", "leotcede"),
        ("IceCreAm", "AceCreIm"),
    ] {
        assert_eq!(reverse_vowels(s.to_string()), expect.to_string());
    }

    for (nums1, nums2, mut intersection) in [
        (vec![1, 2, 2, 1], vec![2, 2], vec![2]),
        (vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![9, 4]),
        (vec![4, 9, 5], vec![1, 2, 3], vec![]),
    ] {
        let mut result = arrays::intersection(nums1.to_vec(), nums2.to_vec());
        result.sort();
        intersection.sort();
        assert_eq!(result, intersection, "{:?}, {:?}", nums1, nums2);
    }
}
