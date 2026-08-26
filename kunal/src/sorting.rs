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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_array() {
        for (mut nums1, nums2, expected) in [
            (vec![ 1, 2, 3, 0, 0, 0 ], vec![ 2, 5, 6 ], vec![1, 2, 2, 3, 5, 6]),
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

}
