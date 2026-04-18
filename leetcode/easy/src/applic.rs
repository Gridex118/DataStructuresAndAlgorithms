pub fn largest_altitude(gains: Vec<i32>) -> i32 {
    let mut altitude = 0;
    let mut max = 0;
    for gain in gains {
        altitude += gain;
        max = max.max(altitude);
    }
    max
}
