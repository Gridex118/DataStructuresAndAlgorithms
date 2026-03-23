pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < m as usize && j < n as usize {
        if nums1[i] > nums2[j] {
            std::mem::swap(&mut nums1[i], &mut nums2[j]);
            let mut k: usize = j;
            while (k < (n - 1) as usize) && (nums2[k] > nums2[k + 1]) {
                nums2.swap(k, k + 1);
                k += 1;
            }
        }
        i += 1;
    }
    while j < n as usize {
        nums1[m as usize + j] = nums2[j];
        j += 1
    }
}
