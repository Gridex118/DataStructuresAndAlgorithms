/// Return the triangle starting at the `row`'th row,
/// where `arr` specifies the contents of the last row
pub fn sum_triangle_from_array(arr: &[i32], row: usize) -> Vec<Vec<i32>> {
    if row == arr.len() {
        return vec![ arr.to_vec() ];
    }
    let mut tail_rows = sum_triangle_from_array(arr, row + 1);
    let lower_row = tail_rows.first().unwrap();
    let this_row = lower_row.windows(2)
        .map(|w| w.iter().sum())
        .collect();
    tail_rows.insert(0, this_row);
    tail_rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_triangle_from_array() {
        assert_eq!(
            sum_triangle_from_array(&[ 1, 2, 3, 4, 5], 1), vec![
                vec![ 48 ],
                vec![ 20, 28 ],
                vec![ 8, 12, 16 ],
                vec![ 3, 5, 7, 9 ],
                vec![ 1, 2, 3, 4, 5 ]
            ]
        );
    }
}
