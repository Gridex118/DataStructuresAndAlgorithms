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

pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut rows = vec![0; m as usize];
    let mut cols = vec![0; n as usize];
    for v in indices {
        rows[v[0] as usize] += 1;
        cols[v[1] as usize] += 1;
    }
    let even_rows = rows.iter().filter(|&&x| x % 2 == 0).count() as i32;
    let even_cols = cols.iter().filter(|&&x| x % 2 == 0).count() as i32;
    let (odd_rows, odd_cols) = (m - even_rows, n - even_cols);
    even_rows * odd_cols + odd_rows * even_cols
}
