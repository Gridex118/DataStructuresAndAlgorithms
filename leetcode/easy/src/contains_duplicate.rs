use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<&i32> = HashSet::from_iter(&nums);
    set.len() != nums.len()
}
