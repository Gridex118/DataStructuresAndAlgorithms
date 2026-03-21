pub fn add(digits: Vec<i32>) -> Vec<i32> {
    if digits.is_empty() { return digits; }
    let mut sum: Vec<i32> = digits;
    for i in (0..sum.len()).rev() {
        sum[i] += 1;
        if sum[i] < 10 {
            return sum;
        }
        sum[i] = 0;
    }
    sum.insert(0, 1);
    sum
}
