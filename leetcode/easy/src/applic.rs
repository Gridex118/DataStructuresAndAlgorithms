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

pub fn _maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut populations = [0; 101];
    for log in logs {
        populations[log[0] as usize - 1950] += 1;
        populations[log[1] as usize - 1950] -= 1;
    }
    let mut current = 0;
    let mut max_population = 0;
    let mut i_max_population = 0;
    for (i, population) in populations.iter().enumerate() {
        current += population;
        if max_population < current {
            max_population = current;
            i_max_population = i;
        }
    }
    i_max_population as i32 + 1950
}

fn _lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix[0].len();
    let mut col_maxs = vec![i32::MIN; n];
    for row in &matrix {
        for (j, &val) in row.iter().enumerate() {
            col_maxs[j] = col_maxs[j].max(val);
        }
    }
    let mut luckies = Vec::new();
    for row in &matrix {
        let (col, &val) = row.iter().enumerate()
            .min_by_key(|&(_, x)| x).unwrap();
        if col_maxs[col] == val {
            luckies.push(val);
        }
    }
    luckies
}

pub struct SolutionMatrixRotation {}
impl SolutionMatrixRotation {
    fn stage_transform(i: usize, j: usize, n: usize, stage: u8) -> (usize, usize) {
        match stage {
            0 => (i, j),
            1 => (j, n - 1 - i),
            2 => (n - 1 - i, n - 1 - j),
            3 => (n - 1 - j, i),
            _ => unreachable!()
        }
    }
    fn are_equal(mat: &[Vec<i32>], target: &[Vec<i32>], stage: u8) -> bool {
        let n = mat.len();
        for (i, row) in mat.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let (i_target, j_target) = Self::stage_transform(i, j, n, stage);
                if *cell != target[i_target][j_target] {
                    return false;
                }
            }
        }
        true
    }
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        (0..4).any(|stage| Self::are_equal(&mat, &target, stage))
    }
}
