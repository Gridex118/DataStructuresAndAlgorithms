pub fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let n = numbers.len();
    assert!(n >= 2);
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    Vec::new()
}

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    // Standard negating at x'th index approach
    // With provision for x <= 0, x > n
    // Maybe handled by setting equal to first positive element
    let n = nums.len();
    let mark_invalid = n as i32 + 1;
    for i in 0..n {
        if nums[i] <= 0 {
            nums[i] = mark_invalid;
        }
    }
    for i in 0..n {
        let j = nums[i].abs() as usize - 1;
        if j >= n { continue; }
        if nums[j] > 0 {
            nums[j] = -nums[j];
        }
    }
    let mut i = 0;
    for &x in &nums {
        if x > 0 { break; }
        i += 1;
    }
    i as i32 + 1
}
