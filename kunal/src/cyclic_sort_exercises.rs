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

/// Given n+1 `nums` in range [1, n], some of which may be duplicated.
/// Find these duplicated numbers.
pub fn find_all_duplicated_numbers(nums: &mut [i32]) -> Vec<i32> {
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
    nums.iter().enumerate()
        .filter_map(|(i, &x)| {
            let j = x as usize - 1;
            (i != j).then_some(x)
        })
        .collect()
}

/// Given n+1 `nums` in range [1, n], One number is duplicated.
/// Find the duplicated and the removed numbers
pub fn set_mismatch(nums: &mut [i32]) -> Option<(i32, i32)> {
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
    for (i, &x) in nums.iter().enumerate() {
        let j = x as usize - 1;
        if i != j {
            return Some((x, i as i32 + 1));
        }
    }
    None
}

/// Given unsorted integers `nums`.
/// Find the smallest missing positive number
pub fn first_missing_positive(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
        if nums[i] > 0 && nums[i] as usize <= n {
            let j = nums[i] as usize - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    for (i, &x) in nums.iter().enumerate() {
        let expected = i as i32 + 1;
        if expected != x {
            return expected;
        }
    }
    n as i32 + 1
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

    #[test]
    fn test_find_all_duplicated_numbers() {
        for (mut nums, duplicated) in [
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2], vec![1]),
            (vec![], vec![]),
        ] {
            let mut calculated = find_all_duplicated_numbers(&mut nums);
            calculated.sort();
            assert_eq!(calculated, duplicated, "Failed for {nums:?}");
        }
    }

    #[test]
    fn test_set_mismatch() {
        for (mut nums, duplicated, removed) in [
            (vec![1, 2, 2, 4], 2, 3),
            (vec![1, 3, 2, 2, 4], 2, 5),
        ] {
            assert_eq!(set_mismatch(&mut nums), Some((duplicated, removed)));
        }
    }

    #[test]
    fn test_first_missing_positive() {
        for (mut nums, missing) in [
            (vec![1, 2, 0], 3),
            (vec![3, 4, -1, 1], 2),
            (vec![6, -9, -3, 1, -5, 8, 3, -2], 2),
        ] {
            assert_eq!(first_missing_positive(&mut nums), missing, "Failed for {nums:?}");
        }
    }

}
