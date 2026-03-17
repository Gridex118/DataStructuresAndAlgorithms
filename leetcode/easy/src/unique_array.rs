pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let Some(max) = nums.iter().copied().max() else {
        return 0;
    };
    let mut current: i32 = nums[0];
    let mut k: i32 = 1;
    // Replace all dups with max()
    for x in nums.iter_mut().skip(1) {
        if *x == current {
            *x = max;
        } else {
            current = *x;
            k += 1;
        }
    }
    nums.sort();
    k
}
