pub fn largest_altitude(gains: Vec<i32>) -> i32 {
    let mut altitude = 0;
    let mut max = 0;
    for gain in gains {
        altitude += gain;
        max = max.max(altitude);
    }
    max
}

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    image.into_iter()
        .map(|v| v.into_iter().rev()
             .map(|x| x ^ 1)
             .collect())
        .collect()
}
