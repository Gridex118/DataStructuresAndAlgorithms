pub fn get_peak(vec: &[i32]) -> usize {
    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    loop {
        let mid: usize = start + (end - start) / 2;
        if start == end {
            break mid;
        }
        if vec[mid] < vec[mid + 1] {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
}
