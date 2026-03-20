fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len().checked_sub(1)?;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if target > nums[mid] {
            left = mid.checked_add(1)?;
        } else {
            right = mid.checked_sub(1)?;
        }
    }
    Some(left)
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    binary_search(&nums, target).unwrap_or_default() as i32
}
