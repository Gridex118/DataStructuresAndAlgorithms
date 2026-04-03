pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    let mut k = 1;
    for i in 1..nums.len() {
        // If this number is already at the correct position, do nothing
        if nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1
        }
    }
    k as i32
}
