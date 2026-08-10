const _RIGHT_HALF_DIAMOND_9_ROWS: &str = "\
*
* *
* * *
* * * *
* * * * *
* * * *
* * *
* *
*";

/// - `n` is a positive odd number, defining the number of rows in the pattern
pub fn pattern_right_half_diamond(n: usize) -> String {
    let row_peak = n / 2 + 1;
    (2..=n).fold(String::from("*"), |pattern, row| {
        let col_max = if row <= row_peak {
            row
        } else {
            n - row + 1
        };
        let this_row = (2..=col_max).fold(String::from("*"), |acc, _| {
            format!("{acc} *")
        });
        format!("{}\n{}", pattern, this_row)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_half_diamond() {
        assert_eq!(
            pattern_right_half_diamond(9),
            String::from(_RIGHT_HALF_DIAMOND_9_ROWS)
        );
    }
}
