use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let nums1: HashSet<i32> = HashSet::from_iter(nums1);
    let nums2: HashSet<i32> = HashSet::from_iter(nums2);
    nums1.intersection(&nums2)
        .cloned().collect()
}

pub fn intersection_no_uniq(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (mut nums1, mut nums2) = (nums1, nums2);
    nums1.sort();
    nums2.sort();
    let mut intersection = Vec::new();
    let (mut p, mut q) = (0, 0);
    while p < nums1.len() && q < nums2.len() {
        if nums1[p] < nums2[q] {
            p += 1;
        } else if nums1[p] > nums2[q] {
            q += 1;
        } else {
            intersection.push(nums1[p]);
            p += 1;
            q += 1;
        }
    }
    intersection
}
