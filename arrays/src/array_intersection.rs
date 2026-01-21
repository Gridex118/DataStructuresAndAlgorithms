pub fn intersection<
        const N: usize, const M: usize
        >(arr1: [i32; N], arr2: [i32; M]) -> Vec<i32> {
    if arr1.len() < arr2.len() {
        intersection(arr2, arr1)
    } else {
        let mut intersection_vec: Vec<i32> = Vec::new();
        let mut it1 = arr1.iter().peekable();
        let mut it2 = arr2.iter().peekable();
        while let (Some(&x), Some(&y)) = (it1.peek(), it2.peek()) {
            if x == y {
                intersection_vec.push(*x);
                it1.next();
                it2.next();
            } else if x < y {
                it1.next();
            } else {
                it2.next();
            }
        }
        if intersection_vec.is_empty() {
            intersection_vec.push(-1);
        }
        intersection_vec
    }
}
