use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let nums1: HashSet<i32> = HashSet::from_iter(nums1);
    let nums2: HashSet<i32> = HashSet::from_iter(nums2);
    nums1.intersection(&nums2)
        .cloned().collect()
}
