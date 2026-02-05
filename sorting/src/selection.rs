pub fn sort(vec: &[i32]) -> Vec<i32> {
    let mut sorted_result: Vec<i32> = vec.to_vec();
    let len: usize = sorted_result.len();
    for i in 0..len {
        let min_i: usize = (i..len).min_by_key(|x| sorted_result[*x])
            .unwrap();
        sorted_result.swap(min_i, i);
    }
    sorted_result
}
