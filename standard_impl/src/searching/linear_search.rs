pub fn linear_search(vec: &[i32], target: i32) -> Option<usize> {
    for (i, x) in vec.iter().enumerate() {
        if *x == target {
            return Some(i);
        }
    }
    None
}
