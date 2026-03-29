fn get_range_str(i: i32, j: i32) -> String {
    if i != j {
        format!("{}->{}", i, j)
    } else {
        i.to_string()
    }
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ranges = Vec::new();
    if nums.is_empty() { return ranges; }
    let mut i = nums[0];
    let mut j_prev = i;
    for &j in &nums[1..] {
        if j != j_prev + 1 {
            ranges.push(get_range_str(i, j_prev));
            i = j;
        }
        j_prev = j;
    }
    ranges.push(get_range_str(i, j_prev));
    ranges
}
