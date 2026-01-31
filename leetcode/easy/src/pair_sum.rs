pub fn sum(nums: &[i32], target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        if *x >= target {
            continue;
        }
        for (j, y) in nums.iter().enumerate().skip(i + 1) {
            if *y == target - x {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![ ]
}
