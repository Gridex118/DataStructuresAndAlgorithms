fn get_3tuple(vec: &mut [i32]) -> (i32, i32, i32) {
    vec.sort();
    (vec[0], vec[1], vec[2])
}

pub fn get_triples(vec: Vec<i32>, sum: i32) -> Vec<(i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32)> = Vec::new();
    for (i, &x) in vec.iter().enumerate() {
        for (j, &y) in vec.iter().enumerate().skip(i + 1) {
            for &z in vec.iter().skip(j + 1) {
                let mut triplet_v: Vec<i32> = vec![x, y, z];
                let triplet: (i32, i32, i32) = get_3tuple(&mut triplet_v);
                if (x + y + z == sum) && !result.contains(&triplet) {
                    result.push(triplet);
                    break;
                }
            }
        }
    }
    result
}
