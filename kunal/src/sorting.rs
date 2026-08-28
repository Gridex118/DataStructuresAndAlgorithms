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

}
