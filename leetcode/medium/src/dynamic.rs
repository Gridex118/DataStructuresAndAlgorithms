pub fn rob_house_two(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    assert!(n > 0);
    if n == 1 {
        nums[0]
    } else if n == 2 {
        nums[1].max(nums[0])
    } else {
        let mut dp1 = vec![0; n - 1];
        dp1[0] = nums[0];
        dp1[1] = nums[1].max(nums[0]);
        for (i, &x) in nums.iter().take(n - 1)
            .enumerate().skip(2) {
                dp1[i] = dp1[i - 1].max(dp1[i - 2] + x);
            }
        let mut dp2 = vec![0; n - 1];
        dp2[0] = nums[1];
        dp2[1] = nums[2].max(nums[1]);
        for (i, &x) in nums.iter().skip(1)
            .enumerate().skip(2) {
                dp2[i] = dp2[i - 1].max(dp2[i - 2] + x);
            }
        dp1[n - 2]
            .max(dp2[n - 2])
    }
}
