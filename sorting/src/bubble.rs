pub fn sort(vec: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec.to_vec();
    let len: usize = result.len();
    for _ in 1..len {
        for j in 1..len {
            if result[j - 1] > result[j] {
                result.swap(j - 1, j);
            }
        }
    }
    result
}
