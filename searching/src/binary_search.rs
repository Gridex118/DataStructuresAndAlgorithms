pub fn search(vec: &[i32], key: i32) -> Option<usize> {
    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    while start <= end {
        let mid: usize = start + (end - start) / 2;
        if vec[mid] == key {
            return Some(mid);
        } else if vec[mid] < key {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    None
}
