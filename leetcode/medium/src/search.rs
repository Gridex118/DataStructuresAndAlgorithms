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
