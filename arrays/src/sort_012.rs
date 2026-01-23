// The Dutch Flag Algorithm
pub fn sort(vec: &mut [i32]) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = vec.len() - 1;
    while j <= k {
        if vec[j] < 1 {
            vec.swap(i, j);
            i += 1;
            j += 1;
        } else if vec[j] > 1 {
            vec.swap(j, k);
            k -= 1;
        } else {
            j += 1;
        }
    }
}
