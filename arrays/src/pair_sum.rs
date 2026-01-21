// Return all pairs in @vec that sum to @sum
pub fn get_pairs(vec: Vec<i32>, sum: i32) -> Vec<(i32, i32)> {
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for (i, x) in vec.iter().enumerate() {
        let y: i32 = sum - x;
        let j: usize = i + 1;
        for a in vec[j..].iter() {
            if y == *a {
                pairs.push((*x, y));
            }
        }
    }
    pairs.sort();
    pairs
}
