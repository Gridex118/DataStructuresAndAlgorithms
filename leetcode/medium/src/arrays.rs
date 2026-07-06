use std::collections::HashMap;

pub fn check_good_subarray(nums: Vec<i32>, k: i32) -> bool {
    let mut prefix_map = HashMap::new();
    prefix_map.insert(0, -1);
    let mut current_sum = 0;
    for (i, &x) in nums.iter().enumerate() {
        let i = i as isize;
        current_sum += x;
        let remainder = current_sum % k;
        if let Some(index_old) = prefix_map.get(&remainder) {
            if i - index_old > 1  {
                return true;
            }
        } else {
            prefix_map.insert(remainder, i);
        }
    }
    false
}
