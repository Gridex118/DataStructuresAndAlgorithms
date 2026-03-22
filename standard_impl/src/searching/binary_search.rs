pub fn binary_search(vec: &[i32], target: i32) -> Option<usize> {
    let mut begin: usize = 0;
    let mut end = vec.len().checked_sub(1)?;
    while begin <= end {
        let mid: usize = begin + (end - begin) / 2;
        if vec[mid] == target {
            return Some(mid);
        } else if vec[mid] > target {
            end = mid - 1;
        } else {
            begin = mid + 1;
        }
    }
    None
}
