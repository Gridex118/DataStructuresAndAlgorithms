use std::collections::{HashSet, HashMap};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<&i32> = HashSet::from_iter(&nums);
    set.len() != nums.len()
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        if let Some(j) = map.insert(x, i)
            && i.abs_diff(j) <= k as usize {
                return true;
            }
    }
    false
}
