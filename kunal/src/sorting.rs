use std::collections::{HashMap, HashSet};

/// Given 2 sorted arrays `nums1` and `nums2`, merge them into a single sorted array
/// - Sort the elements into `nums1`,
///   given that `nums1` contains enough space for elements fo `nums2`
pub fn merge_sorted_array(nums1: &mut [i32], nums2: &[i32]) {
    assert!(nums1.len() >= nums2.len());
    let n = nums2.len();
    let m = nums1.len() - n;
    let (mut i, mut j, mut k) = (m, n, m + n);
    while j > 0 {
        if i > 0 && nums1[i - 1] > nums2[j - 1] {
            nums1[k - 1] = nums1[i - 1];
            i -= 1;
        } else {
            nums1[k - 1] = nums2[j - 1];
            j -= 1;
        }
        k -= 1;
    }
}

/// Given an array `nums` of size n, return the majority element
/// - The majority element is the element that appears more than floor(n/2) times
/// - Assume the majority element always exists
pub fn majority_element(nums: &[i32]) -> i32 {
    let mut answer = nums[0];
    let mut count = 1;
    for &x in nums.iter().skip(1) {
        if x == answer {
            count += 1;
        } else {
            count -= 1;
        }
        if count == 0 {
            answer = x;
            count = 1;
        }
    }
    answer
}

/// Given `nums`, return true if any value appears at least twice in the array
pub fn contains_duplicate(nums: &mut [i32]) -> bool {
    nums.sort_unstable();
    nums.windows(2)
        .any(|w| w[0] == w[1])
}

/// Given `nums` containing n distinct numbers in the range [0, n]
/// Return the missing number in the range
pub fn missing_number(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        if nums[i] != 0 {
            let j = nums[i] as usize - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    nums.iter().position(|&x| x == 0)
        .unwrap() as i32 + 1
}

/// Remove all occurances of `val` in `nums` in place,
/// and return the number of remaining elements
/// - The order of elements may be changed
pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut end = nums.len();
    let mut i = 0;
    while i < end {
        if nums[i] == val {
            for k in i..(end - 1) {
                nums[k] = nums[k + 1];
            }
            end -= 1;
        } else {
            i += 1;
        }
    }
    end as i32
}

/// Given `nums1` and `nums2`, find their intersection
/// Each element in the result must be unique
pub fn intersection(nums1: &mut [i32], nums2: &mut [i32]) -> Vec<i32> {
    let mut answer = Vec::new();
    nums1.sort_unstable();
    nums2.sort_unstable();
    let (m, n) = (nums1.len(), nums2.len());
    let (mut i, mut j) = (0, 0);
    while i < m && j < n {
        if nums1[i] < nums2[j] {
            while i < m && nums1[i] < nums2[j] { i += 1; }
        } else if nums1[i] > nums2[j] {
            while j < n && nums2[j] < nums1[i] { j += 1; }
        } else {
            let common = nums1[i];
            answer.push(common);
            while i < m && nums1[i] == common { i += 1; }
            while j < n && nums2[j] == common { j += 1; }
        }
    }
    answer
}

/// Given `nums1` and `nums2`, return an array of their intersection
/// - Each element in the result must appear as many times as in both arrays
pub fn intersect_2(nums1: &mut [i32], nums2: &mut [i32]) -> Vec<i32> {
    let mut answer = Vec::new();
    nums1.sort_unstable();
    nums2.sort_unstable();
    let (m, n) = (nums1.len(), nums2.len());
    let (mut i, mut j) = (0, 0);
    while i < m && j < n {
        if nums1[i] < nums2[j] {
            while i < m && nums1[i] < nums2[j] { i += 1; }
        } else if nums1[i] > nums2[j] {
            while j < n && nums2[j] < nums1[i] { j += 1; }
        } else {
            let common = nums1[i];
            answer.push(common);
            i += 1;
            j += 1;
        }
    }
    answer
}

/// Return the 3rd distinct maximum number in `nums`
/// - If it does not exist, return the maximum number instead
pub fn third_maximum(nums: &[i32]) -> i32 {
    let (mut x, mut y, mut z) = (None, None, None);
    for &num in nums {
        if [x, y, z].iter().any(|&x| x == Some(num)) {
            continue;
        }
        if x.is_none() || num > x.unwrap() {
            (x, y, z) = (Some(num), x, y);
        } else if y.is_none() || num > y.unwrap() {
            (x, y, z) = (x, Some(num), y);
        } else if z.is_none() || num > z.unwrap() {
            z = Some(num);
        }
    }
    z.unwrap_or(x.unwrap())
}

/// Find the maximum number of content children
/// - Each child `i` has a greed factor `g[i]`, which is the minimum size
/// of a cookie that the child will be content with
/// - Each cookie `j` has a size `s[j]`. If `s[j] >= g[i]`, we can
/// assign the cookie `j` to the child `i`, and the child `i` will be content
pub fn find_content_children(g: &mut [i32], s: &mut [i32]) -> i32 {
    g.sort_unstable();
    s.sort_unstable();
    let (m, n) = (g.len(), s.len());
    let (mut i, mut j) = (0, 0);
    let mut answer = 0;
    while i < m && j < n {
        if g[i] <= s[j] {
            answer += 1;
            i += 1;
        }
        j += 1;
    }
    answer
}

/// Given `nums` of `2n` integers, group these integers into `n` pairs
/// `(ai, bi)` such that sum of `min(ai, bi)` is maximized
/// Return the maximized sum
pub fn array_pair_sum(nums: &mut [i32]) -> i32 {
    nums.sort_unstable();
    nums.iter().step_by(2)
        .sum::<i32>()
}

/// Given `nums`, find 3 numbers whose product is maximum, and return said product
pub fn maximum_product(nums: &mut [i32]) -> i32 {
    assert!(nums.len() >= 3);
    nums.sort_unstable();
    let n = nums.len();
    let largest_all_positive = nums[n - 1] * nums[n - 2] * nums[n - 3];
    let largest_two_negative = nums[0] * nums[1] * nums[n - 1];
    largest_all_positive.max(largest_two_negative)
}

/// Given `nums`, move all the even integers at the beginning of the array
/// followed by all the odd integers, and return the new array
pub fn sort_array_by_parity(nums: &mut [i32]) -> &mut [i32] {
    let n = nums.len();
    let (mut left, mut right) = (0, n);
    while left < right {
        if nums[left] % 2 == 0 {
            left += 1;
        } else if nums[right - 1] % 2 != 0 {
            right -= 1;
        } else {
            nums.swap(left, right - 1);
            left += 1;
            right -= 1;
        }
    }
    nums
}

/// Given `nums`, half of the integers are odd, the other half even
/// Sort the array so that whenever `nums[i]` is odd, `i` is odd and so on
/// Return any array satisfying this condition
pub fn sort_array_by_parity_2(nums: &mut [i32]) -> &mut [i32] {
    let n = nums.len();
    let (mut even, mut odd) = (0, 1);
    while even < n && odd < n {
        if nums[even] % 2 == 0 {
            even += 2;
        } else if nums[odd] % 2 != 0 {
            odd += 2;
        } else {
            nums.swap(odd, even);
            even += 2;
            odd += 2;
        }
    }
    nums
}

/// Given `nums`, return the largest perimeter of the triangle with a non
/// zero area formed from three of these lengts
/// - Return 0 if impossible to form triangles
pub fn largest_perimeter(nums: &mut [i32]) -> i32 {
    let is_triangle = |x, y, z| {
        // We assume (x, y, z) to be sorted
        // If the sum of 2 minimum sides is greater than the max, the other
        // two combinations are trivially true
        x + y > z
    };
    let n = nums.len();
    assert!(n >= 3);
    nums.sort_unstable();
    for window in nums.windows(3).rev() {
        let (x, y, z) = (window[0], window[1], window[2]);
        if is_triangle(x, y, z) {
            return x + y + z;
        }
    }
    0
}

/// Given `nums` sorted in non-decreasing order, return an array of the squares
/// of each number sorted in non-decreasing order
/// - `O(n)` approach other than "squaring all elements and sorting the new array"
pub fn sorted_squares(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut answer = vec![0; n];
    let (mut left, mut right, mut insert) = (0, n - 1, n);
    while insert > 0 {
        let left_sq = nums[left] * nums[left];
        let right_sq = nums[right] * nums[right];
        if left_sq > right_sq {
            answer[insert - 1] = left_sq;
            left += 1;
        } else {
            answer[insert - 1] = right_sq;
            right -= 1;
        }
        insert -= 1;
    }
    answer
}

/// There is a `rows x cols` matrix, and you are on the cell `c = (r_center, c_center)`
/// Return the coordinates of all cells in the matrix, sorted by their distance from `c`
/// from smallest to largest distance.
pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let distance = |x: &[i32]| {
        (x[0] - r_center).abs() + (x[1] - c_center).abs()
    };
    let mut mat = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            mat.push(vec![ i, j ]);
        }
    }
    mat.sort_unstable_by_key(|v| distance(&v));
    mat
}

/// A school is trying to take an annual photo of all the students. The students
/// are asked to stand in a single line in non decreasing order by height.
/// Let this ordering be represented by the integer array `expected` where
/// `expected[i]` is the expected height of the `i`th student in line.
/// You are given an array `heights` representing the current order that
/// the students are standing in. Each `height[i]` is the height of the
/// `i`th student in line (0-indexed).
/// Return the number of indices where `heights[i] != expected[i]`.
pub fn height_checker(heights: &[i32]) -> i32 {
    let mut expected = heights.to_vec();
    expected.sort_unstable();
    heights.iter()
        .zip(expected)
        .filter(|&(&x, y)| x != y)
        .count() as i32
}

/// Sort the elements of arr1 such that the relative ordering
/// of items in arr1 are the same as in arr2.
/// Elements that do not appear in arr2 should be placed at the end of
/// arr1 in ascending order.
pub fn relative_sort_array(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let arr1 = Vec::from(arr1);
    let mut arr1_count = HashMap::new();
    for x in arr1 {
        *arr1_count.entry(x).or_insert(0) += 1;
    }
    let mut result = Vec::new();
    for &x in arr2 {
        if let Some(freq) = arr1_count.remove(&x) {
            for _ in 0..freq {
                result.push(x);
            }
        }
    }
    let mut remaining: Vec<i32> = arr1_count.keys().cloned().collect();
    remaining.sort();
    for x in remaining {
        for _ in 0..arr1_count[&x] {
            result.push(x);
        }
    }
    result
}

/// Given an array of distinct integers arr, find all pairs of
/// elements with the minimum absolute difference of any two elements.
/// Return a list of pairs in ascending order(with respect to pairs)
pub fn minimum_absolute_difference(arr: &mut [i32]) -> Vec<Vec<i32>> {
    arr.sort();
    let min_abs_diff = arr.windows(2)
        .map(|w| w[1] - w[0])
        .min()
        .unwrap();
    arr.windows(2)
        .filter(|w| w[1] - w[0] == min_abs_diff)
        .map(|w| w.to_vec()).collect()
}

/// Given an array of integers `arr`, replace each element with its rank.
/// The rank represents how large the element is. The rank has the following rules:
///
/// - Rank is an integer starting from 1.
///
/// - The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
///
/// - Rank should be as small as possible.
pub fn array_rank_transform(arr: &[i32]) -> Vec<i32> {
    let mut arr_sorted = arr.to_vec();
    arr_sorted.sort();
    arr_sorted.dedup();
    arr.iter()
        .map(|val| (arr_sorted.binary_search(val).unwrap() as i32) + 1)
        .collect()
}

/// You are given an integer array arr. Sort the integers in the array
/// in ascending order by the number of 1's in their binary
/// representation and in case of two or more integers have the same
/// number of 1's you have to sort them in ascending order. Return the
/// array after sorting it.
pub fn sort_by_bits(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.iter()
        .map(|&b| (b.count_ones(), b))
        .collect::<Vec<(u32, i32)>>();
    arr.sort();
    arr.iter().map(|t| t.1).collect()
}

/// Given the array `nums`, for each `nums[i]` find out how many numbers
/// in the array are strictly smaller than it.
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut nums_sorted = nums.to_vec();
    nums_sorted.sort();
    nums.iter()
        .map(|x| {
            nums_sorted.partition_point(|y| y < x) as i32
        })
        .collect()
}

/// You are given an array of integers `nums`. Choose two different
/// indices `i` and `j` of that array. Return the maximum value of
/// `(nums[i] - 1) * (nums[j] - 1)`
/// - 2 <= nums.length <= 500
///
/// - 1 <= nums[i] <= 10^3
pub fn max_product(nums: &[i32]) -> i32 {
    let mut max_first = 0;
    let mut max_second = 0;
    for &x in nums {
        if x > max_first {
            (max_second, max_first) = (max_first, x);
        } else if x > max_second {
            max_second = x;
        }
    }
    (max_first - 1) * (max_second - 1)
}

/// Return the average salary of employees excluding
/// the minimum and maximum salary.
pub fn average_salary(salary: &[i32]) -> f64 {
    let min = salary.iter().min().unwrap();
    let max = salary.iter().max().unwrap();
    let mut sum = 0;
    for x in salary {
        if x != min && x != max {
            sum += x;
        }
    }
    sum as f64 / (salary.len() - 2) as f64
}

/// You are given two integer arrays of equal length `target` and `array`.
/// In one step, you can select any non-empty subarray of `arr` and reverse it.
/// You are allowed to make any number of steps.
/// Return true if you can make `arr` equal to `target` or false otherwise
pub fn can_be_equal(target: &mut [i32], arr: &mut [i32]) -> bool {
    target.sort();
    arr.sort();
    target == arr
}

/// Given an array of integers `nums`, sort the array in increasing order
/// based on the frequency of the values. If multiple values have the
/// same frequency, sort them in decreasing order
pub fn frequency_sort(nums: &[i32]) -> Vec<i32> {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &x in nums {
        *(frequency_map.entry(x).or_insert(0)) += 1;
    }
    let mut sorted = nums.to_vec();
    sorted.sort_by(|a, b| {
        match frequency_map[a].cmp(&frequency_map[b]) {
            std::cmp::Ordering::Equal => b.cmp(&a),
            ord => ord
        }
    });
    sorted
}

/// You are given an array `nums` of non-negative integers. `nums` is
/// considered special if there exists a number x such that there are
/// exactly x numbers in `nums` that are greater than or equal to x.
/// 
/// Return x if the array is special, otherwise, return -1. x is unique.
pub fn special_array(nums: &mut [i32]) -> i32 {
    nums.sort();
    for &num in nums.iter().rev() {
        let x = nums.partition_point(|&y| y <= num) as i32;
        if x <= num {
            return x;
        }
    }
    -1
}

/// Given an array `nums` of n integers where nums[i] is in the range
/// [1, n], return an array of all the integers in the range [1, n]
/// [1, n] that do not appear in `nums`
pub fn find_disappeared_numbers(nums: &mut [i32]) -> Vec<i32> {
    let mut i = 0;
    let n = nums.len();
    while i < n {
        let j = nums[i] as usize - 1;
        if nums[j] != nums[i] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }
    nums.iter().enumerate()
        .filter(|&(i, &x)| i as i32 + 1 != x)
        .map(|(i, _)| i as i32 + 1)
        .collect()
}

/// You have a set of integers s, which originally contains all the
/// numbers from 1 to n. Unfortunately, due to some error, one of the
/// numbers in s got duplicated to another number in the set, which
/// results in repetition of one number and loss of another number.
///
/// You are given an integer array `nums` representing the data status of
/// this set after the error. Find the number that occurs twice and the
/// number that is missing and return them in the form of an array.
pub fn find_error_nums(nums: &mut [i32]) -> Vec<i32> {
    let mut i = 0;
    let n = nums.len();
    while i < n {
        let j = nums[i] as usize - 1;
        if nums[j] != nums[i] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }
    for (i, &x) in nums.iter().enumerate() {
        let i = i as i32;
        if i + 1 != x {
            return vec![x, i + 1];
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_array() {
        for (mut nums1, nums2, expected) in [
            (vec![ 1, 2, 3, 0, 0, 0 ], vec![ 2, 5, 6 ], vec![ 1, 2, 2, 3, 5, 6 ]),
            (vec![ 1 ], vec![], vec![ 1 ]),
            (vec![ 0 ], vec![ 1 ], vec![ 1 ]),
        ] {
            merge_sorted_array(&mut nums1, &nums2);
            assert_eq!(nums1, expected, "Failed for {nums1:?}");
        }
    }

    #[test]
    fn test_majority_element() {
        for (nums, expected) in [
            (vec![ 3, 2, 3 ], 3),
            (vec![ 2, 2, 1, 1, 1, 2, 2 ], 2),
            (vec![ 1 ], 1),
        ] {
            assert_eq!(majority_element(&nums), expected, "{nums:?}");
        }
    }

    #[test]
    fn test_contains_duplicate() {
        assert!(contains_duplicate(&mut [ 1, 2, 3, 1 ]));
        assert!(!contains_duplicate(&mut [ 1, 2, 3, 4 ]));
        assert!(contains_duplicate(&mut [ 1, 1, 1, 3, 3, 4, 3, 2, 4, 2 ]));
    }

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(&mut [ 3, 0, 1 ]), 2);
        assert_eq!(missing_number(&mut [ 0, 1 ]), 2);
        assert_eq!(missing_number(&mut [ 9,6,4,2,3,5,7,0,1 ]), 8);
    }

    #[test]
    fn test_remove_element() {
        for (nums, val, expected) in [
            (vec![ 3, 2, 2, 3 ], 3, vec![ 2, 2 ]),
            (vec![ 0, 1, 2, 2, 3, 0, 4, 2 ], 2, vec![ 0, 1, 3, 0, 4 ]),
        ] {
            let mut nums_m = nums.to_vec();
            let removed = remove_element(&mut nums_m, val);
            assert_eq!(removed, expected.len() as i32,
                       "Wrong count for {nums:?}");
            for (i, &x) in expected.iter().enumerate() {
                assert_eq!(x, nums_m[i], "Failed for index {i} in {nums_m:?} vs {expected:?}");
            }
        }
    }

    #[test]
    fn test_intersection() {
        for (mut nums1, mut nums2, expected) in [
            (vec![ 1, 2, 2, 1 ], vec![ 2, 2 ], vec![ 2 ]),
            (vec![ 4, 9, 5 ], vec![ 9, 4, 9, 8, 4 ], vec![ 4, 9 ]),
        ] {
            assert_eq!(intersection(&mut nums1, &mut nums2), expected,
                       "Failed for {nums1:?}, {nums2:?}");
        }
    }

    #[test]
    fn test_intersection_2() {
        for (mut nums1, mut nums2, expected) in [
            (vec![ 1, 2, 2, 1 ], vec![ 2, 2 ], vec![ 2, 2 ]),
            (vec![ 4, 9, 5 ], vec![ 9, 4, 9, 8, 4 ], vec![ 4, 9 ]),
        ] {
            let mut answer = intersect_2(&mut nums1, &mut nums2);
            answer.sort_unstable();
            assert_eq!(answer, expected, "Failed for {nums1:?}, {nums2:?}");
        }
    }

    #[test]
    fn test_third_maximum() {
        for (nums, expected) in [
            (vec![ 3, 2, 1 ], 1),
            (vec![ 1, 2 ], 2),
            (vec![ 2, 3, 2, 1 ], 1),
        ] {
            assert_eq!(third_maximum(&nums), expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_find_content_children() {
        for (mut greed, mut sizes, expected) in [
            (vec![ 1, 2, 3 ], vec![ 1, 1 ], 1),
            (vec![ 1, 2 ], vec![ 1, 2, 3 ],  2),
        ] {
            assert_eq!(find_content_children(&mut greed, &mut sizes), expected,
                       "Failed for {greed:?} x {sizes:?}");
        }
    }

    #[test]
    fn test_array_pair_sum() {
        for (mut nums, expected) in [
            (vec![ 1, 4, 3, 2 ], 4),
            (vec![ 6, 2, 6, 5, 1, 2 ], 9),
        ] {
            assert_eq!(array_pair_sum(&mut nums), expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_maximum_product() {
        for (mut nums, expected) in [
            (vec![ 1, 2, 3 ], 6),
            (vec![ 1, 2, 3, 4 ], 24),
            (vec![ -1, -2, -3 ], -6),
        ] {
            assert_eq!(maximum_product(&mut nums), expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_sort_array_by_parity() {
        for (nums, mut expected) in [
            (vec![ 3, 1, 2, 4 ], vec![ 2, 4, 3, 1 ]),
            (vec![ 0 ], vec![ 0 ]),
        ] {
            let mut nums_m = nums.to_vec();
            let parity_sorted: &mut [i32] = sort_array_by_parity(&mut nums_m);
            parity_sorted.sort_unstable();
            expected.sort_unstable();
            assert_eq!(parity_sorted, expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_sort_array_by_parity_2() {
        for (nums, mut expected) in [
            (vec![ 4, 2, 5, 7 ], vec![ 4, 5, 2, 7 ]),
            (vec![ 2, 3 ], vec![ 2, 3 ]),
        ] {
            let mut nums_m = nums.to_vec();
            let parity_sorted = sort_array_by_parity_2(&mut nums_m);
            parity_sorted.sort_unstable();
            expected.sort_unstable();
            assert_eq!(parity_sorted, expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_largest_perimeter() {
        for (mut nums, expected) in [
            (vec![ 2, 1, 2 ], 5),
            (vec![ 1, 2, 1, 10 ], 0),
        ] {
            assert_eq!(largest_perimeter(&mut nums), expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_sorted_squares() {
        for (nums, expected) in [
            (vec![ -4, -1, 0, 3, 10 ], vec![ 0, 1, 9, 16, 100 ]),
            (vec![ -7, -3, 2, 3, 11 ], vec![ 4, 9, 9, 49, 121 ]),
        ] {
            assert_eq!(sorted_squares(&nums), expected, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_all_cells_dist_order() {
        for (r, c, r_center, c_center, expected) in [
            (1, 2, 0, 0, vec![
                vec![ 0, 0 ], vec![ 0, 1 ]
            ]),
            (2, 2, 0, 1, vec![
                vec![ 0, 1 ], vec![ 0, 0 ],
                vec![ 1, 1 ], vec![ 1, 0 ]
            ]),
        ] {
            assert_eq!(all_cells_dist_order(r, c, r_center, c_center), expected,
                       "Failed for {r}, {c}, {r_center}, {c_center}");
        }
    }

    #[test]
    fn test_height_checker() {
        for (heights, expected) in [
            (vec![ 1,1,4,2,1,3 ], 3),
            (vec![ 5, 1, 2, 3, 4 ], 5),
            (vec![ 1, 2, 3, 4, 5 ], 0),
        ] {
            assert_eq!(height_checker(&heights), expected,
                       "Failed for {heights:?}");
        }
    }

    #[test]
    fn test_relative_sort_array() {
        for (arr1, arr2, expected) in [
            (vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
             vec![2, 1, 4, 3, 9, 6],
             vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]),
            (vec![28, 6, 22, 8, 44, 17],
             vec![22, 28, 8, 6],
             vec![22, 28, 8, 6, 17, 44]),
        ] {
            assert_eq!(relative_sort_array(&arr1, &arr2), expected,
                       "Failed for {arr1:?}, {arr2:?}")
        }
    }

    #[test]
    fn test_minimum_absolute_difference() {
        for (mut arr1, expected) in [
            (vec![ 4, 2, 1, 3 ], vec![ [1, 2], [2, 3], [3, 4] ]),
            (vec![ 1, 3, 6, 10, 15 ], vec![ [1, 3] ]),
        ] {
            assert_eq!(minimum_absolute_difference(&mut arr1), expected);
        }
    }

    #[test]
    fn test_array_rank_transform() {
        for (arr, expected) in [
            (vec![ 40, 10, 20, 30 ], vec![ 4, 1, 2, 3 ]),
            (vec![ 100, 100, 100 ], vec![ 1, 1, 1 ]),
            (vec![ 37, 12, 28, 9, 100, 56, 80, 5, 12 ], vec![ 5, 3, 4, 2, 8, 6, 7, 1, 3 ])
        ] {
            assert_eq!(array_rank_transform(&arr), expected);
        }
    }

    #[test]
    fn test_sort_by_bits() {
        for (arr, expected) in [
            (vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8 ],
             vec![ 0, 1, 2, 4, 8, 3, 5, 6, 7 ]),
            (vec![ 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1 ],
             vec![ 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024 ]),
        ] {
            assert_eq!(sort_by_bits(&arr), expected);
        }
    }

    #[test]
    fn test_smaller_numbers_than_current() {
        for (arr, expected) in [
            (vec![ 8, 1, 2, 2, 3 ], vec![ 4, 0, 1, 1, 3 ]),
            (vec![ 6, 5, 4, 8 ], vec![ 2, 1, 0, 3 ]),
            (vec![ 7, 7, 7, 7 ], vec![ 0, 0, 0, 0 ]),
        ] {
            assert_eq!(smaller_numbers_than_current(&arr), expected);
        }
    }

    #[test]
    fn test_max_product() {
        for (nums, expected) in [
            (vec![3, 4, 5, 2], 12),
            (vec![1, 5, 4, 5], 16)
        ] {
            assert_eq!(max_product(&nums), expected);
        }
    }

    #[test]
    fn test_average_salary() {
        for (salary, expected) in [
            (vec![ 4000, 3000, 1000, 2000 ], 2500.0_f64),
            (vec![ 1000, 2000, 3000 ], 2000.0_f64),
        ] {
            assert_eq!(average_salary(&salary), expected);
        }
    }

    #[test]
    fn test_can_be_equal() {
        for (mut target, mut arr, expected) in [
            (vec![ 1, 2, 3, 4 ], vec![ 2, 4, 1, 3 ], true),
            (vec![ 7 ], vec![ 7 ], true),
            (vec![ 3, 7, 9 ], vec![ 3, 7, 11 ], false),
        ] {
            assert_eq!(can_be_equal(&mut target, &mut arr), expected,
                       "Failed for {target:?}, {arr:?}");
        }
    }

    #[test]
    fn test_frequency_sort() {
        for (nums, expected) in [
            (vec![ 1, 1, 2, 2, 2, 3 ], vec![ 3, 1, 1, 2, 2, 2 ]),
            (vec![ 2, 3, 1, 3, 2 ], vec![ 1, 3, 3, 2, 2 ]),
            (vec![ -1, 1, -6, 4, 5, -6, 1, 4, 1 ],
             vec![ 5, -1, 4, 4, -6, -6, 1, 1, 1 ]),
        ] {
            assert_eq!(frequency_sort(&nums), expected,
                       "Frequency Sort failed for {nums:?}");
        }
    }

    #[test]
    fn test_special_array() {
        for (mut nums, expected) in [
            (vec![ 3, 5 ], 2),
            (vec![ 0, 0 ], -1),
            (vec![ 0, 4, 3, 0, 4 ], 3)
        ] {
            assert_eq!(special_array(&mut nums), expected,
                       "Failed special array for {nums:?}")
        }
    }

    #[test]
    fn test_find_disappeared_numbers() {
        for (nums, expected) in [
            (vec![ 4, 3, 2, 7, 8, 2, 3, 1 ], vec![ 5, 6 ]),
            (vec![ 1, 1 ], vec![ 2 ])
        ] {
            let mut nums_copy = nums.to_vec();
            assert_eq!(find_disappeared_numbers(&mut nums_copy), expected,
                       "Failed to find disappeared numbers for {nums:?}");
        }
    }

    #[test]
    fn test_find_error_nums() {
        for (nums, expected) in [
            (vec![ 1, 2, 2, 4 ], vec![ 2, 3 ]),
            (vec![ 1, 1 ], vec![ 1, 2 ])
        ] {
            let mut nums_mut = nums.to_vec();
            assert_eq!(find_error_nums(&mut nums_mut), expected,
                       "Failed find_error_nums for {nums:?}");
        }
    }

}
