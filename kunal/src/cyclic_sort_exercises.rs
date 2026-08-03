/// Given unique `nums` in range [0, n], exactly one of which is missing.
/// Find that missing number.
pub fn missing_number(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        let j = nums[i] as usize;
        if (j < n) && (nums[i] != nums[j]) {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }
    for (i, &x) in nums.iter().enumerate() {
        if i as i32 != x {
            return i as i32;
        }
    }
    n as i32
}

/// Given a sequence `nums` with numbers in range [1, n] (has duplicates).
/// Returns all missing numbers in said range.
pub fn find_disappeared_numbers(nums: &mut [i32]) -> Vec<i32> {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        let j = nums[i] as usize - 1;
        if nums[i] != nums[j] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }
    let mut dissappeared = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        if x != i as i32 + 1 {
            dissappeared.push(i as i32 + 1);
        }
    }
    dissappeared
}

/// Given n+1 `nums` in range [1, n], exactly one of which is duplicated.
/// Find this duplicated number.
///
/// **Theory:** If number to be slotted is already positioned, this number is duplicated.
pub fn find_duplicated(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        let j = nums[i] as usize - 1;
        if i != j {
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                return nums[i];
            }
        } else {
            i += 1;
        }
    }
    *nums.last().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        for (mut nums, missing) in [
            (vec![0, 3, 1], 2),
            (vec![9, 6, 4, 3, 2, 5, 7, 1, 0], 8),
            (vec![0], 1),
        ] {
            assert_eq!(missing_number(&mut nums), missing);
        }
    }

    #[test]
    fn test_find_dissapeared_numbers() {
        for (mut nums, dissapeared) in [
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![5, 6]),
            (vec![1, 1], vec![2]),
        ] {
            assert_eq!(find_disappeared_numbers(&mut nums), dissapeared, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_find_duplicated_number() {
        for (mut nums, duplicated) in [
            (vec![1, 3, 4, 2, 2], 2),
            (vec![3, 1, 3, 4, 2], 3),
            (vec![2, 2], 2),
        ] {
            assert_eq!(find_duplicated(&mut nums), duplicated, "Failed for {nums:?}");
        }
    }

}
