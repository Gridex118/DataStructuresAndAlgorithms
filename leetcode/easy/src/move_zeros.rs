pub fn move_zeros(nums: &mut [i32]) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}
