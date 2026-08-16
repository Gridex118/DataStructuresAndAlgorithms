/// Return the triangle starting at the `row`'th row,
/// where `arr` specifies the contents of the last row
pub fn sum_triangle_from_array(arr: &[i32], row: usize) -> Vec<Vec<i32>> {
    if row == arr.len() {
        return vec![ arr.to_vec() ];
    }
    let mut tail_rows = sum_triangle_from_array(arr, row + 1);
    let lower_row = tail_rows.first().unwrap();
    let this_row = lower_row.windows(2)
        .map(|w| w.iter().sum())
        .collect();
    tail_rows.insert(0, this_row);
    tail_rows
}

/// Returns the `(minimum, maximum)` element pair in the given `arr`
/// if the array is non empty. Otherwise, returns `None`
pub fn array_min_and_max(arr: &[i32]) -> Option<(i32, i32)> {
    if arr.is_empty() {
        return None;
    } else if arr.len() == 1 {
        return Some((arr[0], arr[1]));
    }
    let n = arr.len();
    let head = arr[n - 1];
    let (min, max) = array_min_and_max(&arr[..(n - 1)])
        .unwrap();
    Some((min.min(head), max.max(head)))
}

/// Search for `target` in `arr`, and return the index at which it is found
/// - Returns `None` if `target` is not found
pub fn binary_search(arr: &[i32], target: i32, low: usize, high: usize) -> Option<usize> {
    if low >= high {
        return None
    }
    let mid = low + (high - low) / 2;
    if arr[mid] == target {
        return Some(mid)
    } else if arr[mid] > target {
        return binary_search(arr, target, low, mid);
    } else {
        return binary_search(arr, target, mid + 1, high);
    }
}

/// Return the first uppercase letter in `s`
/// - Returns `None` is `s` is empty or no uppercase letter is found
pub fn first_uppercase_letter(s: &str) -> Option<char> {
    if s.is_empty() {
        return None;
    }
    let c = s.chars().nth(0).unwrap();
    if c.is_uppercase() {
        Some(c)
    } else {
        first_uppercase_letter(&s[1..])
    }
}

/// Returns the reverse of the string `s`
pub fn reverse_string(s: &str) -> String {
    match s.chars().nth(0) {
        None => String::new(),
        Some(first) => {
            let mut tail = reverse_string(&s[1..]);
            tail.push(first);
            tail
        }
    }
}

/// Returns a string with numbers 1..`n` separated by space
/// - Returns empty string if `n` = 0
pub fn print_till_n(n: usize) -> String {
    if n == 0 {
        String::new()
    } else if n == 1 {
        String::from("1")
    } else {
        let mut s = print_till_n(n - 1);
        s.push(' ');
        s.push_str(&n.to_string());
        s
    }
}

/// Returns the `nth` fibonacci number
pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

/// Returns the `nth` special fibonacci:
/// - `a`, `b` are the values for F_0 and F_1 respectively
/// - `memo` is a sequence holding the previous values
pub fn special_fibonacci(a: i64, b: i64, n: usize, memo: &mut [Option<i64>]) -> i64 {
    if let Some(result) = memo[n] {
        return result;
    }
    memo[n] = Some(if n == 0 {
        a
    } else if n == 1 {
        b
    } else {
        special_fibonacci(a, b, n - 2, memo)
            ^ special_fibonacci(a, b, n - 1, memo)
    });
    memo[n].unwrap()
}

/// Returns the length of string `s`
pub fn string_length(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        1 + string_length(&s[1..])
    }
}

/// - G_1 = `a`, G_2 = `b`, G_3 = `c`
/// - G_n = G_(n - 3) + G_(n - 2) + G_(n - 1)
pub fn geekonacci(a: i64, b: i64, c: i64, n: i64) -> i64 {
    if n == 1 {
        a
    } else if n == 2 {
        b
    } else if n == 3 {
        c
    } else {
        geekonacci(a, b, c, n - 3)
            + geekonacci(a, b, c, n - 2)
            + geekonacci(a, b, c, n - 1)
    }
}

/// Sort given array in ascending order
pub fn bubble_sort(arr: &mut [i32], n: usize) {
    if n < 2 { return; }
    let mut swapped = false;
    for i in 1..n {
        if arr[i] < arr[i - 1] {
            arr.swap(i, i - 1);
            swapped = true;
        }
    }
    if swapped { bubble_sort(arr, n - 1); }
}

/// Sort given array in ascending order
pub fn insertion_sort(arr: &mut [i32], n: usize) {
    if n < 2 { return; }
    insertion_sort(arr, n - 1);
    let key = arr[n - 1];
    let mut j = n - 1;
    while j > 0 && arr[j - 1] > key {
        arr[j] = arr[j - 1];
        j -= 1;
    }
    arr[j] = key;
}

/// Find the sum of digits in `n`
pub fn sum_of_digits(num: u32) -> u32 {
    if num / 10 == 0 {
        num
    } else {
        (num % 10) + sum_of_digits(num / 10)
    }
}

/// Find the product of `x` and `y`
pub fn product(mut x: u32, mut y: u32) -> u32 {
    // Add `x` to itself, `y` times
    // `y` being the smaller number reduces the number of recursion calls
    if x < y { std::mem::swap(&mut x, &mut y); }
    if y == 0 {
        0
    } else {
        x + product(x, y - 1)
    }
}

/// Check if `n` is a prime number or not
fn is_prime_go(n: u32, i: u32) -> bool {
    if n < 2 {
        false
    } else if i >= n {
        true
    } else {
        (n % i != 0) && is_prime_go(n, i + 1)
    }
}
pub fn is_prime(n: u32) -> bool {
    is_prime_go(n, 2)
}

/// Check if `arr` is sorted in ascending order or not
pub fn is_sorted(arr: &[i32]) -> bool {
    if arr.len() < 2 {
        return true;
    } else {
        (arr[0] <= arr[1]) && is_sorted(&arr[1..])
    }
}

/// Remove consecutive duplicate characters in string `s`
pub fn remove_consecutive_duplicates(s: &str) -> String {
    if s.len() < 2 {
        String::from(s)
    } else {
        let n = s.len();
        let mut result = remove_consecutive_duplicates(&s[..(n - 1)]);
        let s = s.as_bytes();
        if s[n - 1] != s[n - 2] {
            result.push(s[n - 1] as char);
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_triangle_from_array() {
        assert_eq!(
            sum_triangle_from_array(&[ 1, 2, 3, 4, 5], 1), vec![
                vec![ 48 ],
                vec![ 20, 28 ],
                vec![ 8, 12, 16 ],
                vec![ 3, 5, 7, 9 ],
                vec![ 1, 2, 3, 4, 5 ]
            ]
        );
    }

    #[test]
    fn test_array_min_and_max() {
        assert_eq!(
            array_min_and_max(&[1, 4, 3, -5, -4, 8, 6]),
            Some((-5, 8))
        );
    }

    #[test]
    fn test_binary_search() {
        for (arr, target, expected) in [
            ([ -1,0,3,5,9,12 ], 9, Some(4)),
            ([ -1,0,3,5,9,12 ], 2, None),
        ] {
            assert_eq!(
                binary_search(&arr, target, 0, arr.len()),
                expected
            );
        }
    }

    #[test]
    fn test_first_uppercase_letter() {
        for (s, expected) in [
            ("geeksforgeeKs", Some('K')),
            ("geekS", Some('S')),
            ("geeks", None),
        ] {
            assert_eq!(first_uppercase_letter(s), expected);
        }
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("1234"), String::from("4321"));
        assert_eq!(reverse_string(""), String::from(""));
        assert_eq!(reverse_string("1"), String::from("1"));
    }

    #[test]
    fn test_print_till_n() {
        assert_eq!(print_till_n(0), "");
        assert_eq!(print_till_n(1), "1");
        assert_eq!(print_till_n(5), "1 2 3 4 5");
        assert_eq!(print_till_n(11), "1 2 3 4 5 6 7 8 9 10 11");
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(8), 21);
    }

    #[test]
    fn test_special_fibonacci() {
        let mut memo = [ None; 16 ];
        assert_eq!(special_fibonacci(86, 77, 15, &mut memo), 86);
        let mut memo = [ None; 128 ];
        assert_eq!(special_fibonacci(93, 35, 86, &mut memo), 126);
    }

    #[test]
    fn test_string_length() {
        assert_eq!(string_length(""), 0);
        assert_eq!(string_length("1"), 1);
        assert_eq!(string_length("abcd"), 4);
        assert_eq!(string_length("GEEKSFORGEEKS"), 13);
    }

    #[test]
    fn test_geekonacci() {
        assert_eq!(geekonacci(1, 3, 2, 4), 6);
        assert_eq!(geekonacci(1, 3, 2, 6), 19);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![ 5, 1, 4, 2, 8, ];
        assert!(!arr.is_sorted());
        let n = arr.len();
        bubble_sort(&mut arr, n);
        assert!(arr.is_sorted());

        let mut arr = vec![];
        assert!(arr.is_sorted());
        let n = arr.len();
        bubble_sort(&mut arr, n);
        assert!(arr.is_sorted());
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![ 5, 1, 4, 2, 8, ];
        assert!(!arr.is_sorted());
        let n = arr.len();
        insertion_sort(&mut arr, n);
        assert!(arr.is_sorted());

        let mut arr = vec![];
        assert!(arr.is_sorted());
        let n = arr.len();
        insertion_sort(&mut arr, n);
        assert!(arr.is_sorted());
    }

    #[test]
    fn test_sum_of_digits() {
        for (num, sum) in [
            (12345, 15), (45632, 20),
            (0, 0), (1, 1)
        ] {
            assert_eq!(sum_of_digits(num), sum, "Failed for {num}");
        }
    }

    #[test]
    fn test_product() {
        for (x, y, z) in [
            (5, 2, 10), (100, 5, 500),
        ] {
            assert_eq!(product(x, y), z, "Product of {x}, {y} failed");
        }
    }

    #[test]
    fn test_prime() {
        assert!(!is_prime(1), "Failed for 1");
        assert!(is_prime(2), "Failed for 2");
        assert!(is_prime(11), "Failed for 11");
        assert!(!is_prime(15), "Failed for 15");
    }

    #[test]
    fn test_name() {
        assert!(is_sorted(&[10, 20, 30, 40, 50]));
        assert!(is_sorted(&[1]));
        assert!(is_sorted(&[]));
        assert!(!is_sorted(&[90, 80, 100, 70, 40, 30]));
        assert!(!is_sorted(&[100, 20, 30, 40, 50]));
    }

    #[test]
    fn test_remove_consecutive_duplicates() {
        for (s, expected) in [
            ("aaaaabbbbbb", String::from("ab")),
            ("geeksforgeeks", String::from("geksforgeks")),
        ] {
            assert_eq!(remove_consecutive_duplicates(s), expected,
                       "Failed for {s}");
        }
    }


}
