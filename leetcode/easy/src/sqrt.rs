pub fn sqrt(x: i32) -> i32 {
    let mut start: i32 = 1;
    let mut end: i32 = x;
    while start <= end {
        let mid: i32 = start + (end - start) / 2;
        if mid == x / mid {
            return mid;
        } else if mid > x / mid {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    start - 1
}
