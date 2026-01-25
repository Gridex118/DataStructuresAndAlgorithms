// Find the minimum point in a zig zag
pub fn get_pivot_index(vec: &[i32]) -> usize {
    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    let rightmost: i32 = vec[end];
    while start < end {
        let mid: usize = start + (end - start) / 2;
        if vec[mid] > rightmost {
            start = mid + 1;
        } else if vec[mid] < rightmost {
            end = mid;
        }
    }
    start
}
