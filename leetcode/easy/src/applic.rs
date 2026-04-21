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

fn _kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max()
        .unwrap();
    candies.iter()
        .map(|c| c + extra_candies >= max_candies)
        .collect()
}

fn _diagnol_sum_matrix(mat: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let n = mat.len();
    for i in 0..n {
        sum += mat[i][i] + mat[i][n - i - 1];
    }
    if !n.is_multiple_of(2) {
        sum -= mat[n / 2][n / 2];
    }
    sum
}

fn _transpose_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let mut transpose = vec![vec![0; m]; n];
    for row in 0..n {
        for col in 0..m {
            transpose[row][col] = mat[col][row];
        }
    }
    transpose
}

pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut sum = Vec::new();
    let mut carry = 0;
    while !num.is_empty() || carry == 1 || k > 0 {
        let this_digit = num.pop().unwrap_or(0) + k % 10 + carry;
        sum.push(this_digit % 10);
        carry = this_digit / 10;
        k /= 10;
    }
    sum.reverse();
    sum
}
