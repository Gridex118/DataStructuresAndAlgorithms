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

const _FULL_DIAMOND_9_ROWS: &str = "\
\x20   *\n\
\x20  * *\n\
\x20 * * *\n\
\x20* * * *\n\
   * * * * *\n\
\x20* * * *\n\
\x20 * * *\n\
\x20  * *\n\
\x20   *";

const _TOP_HALF_DIAMOND_NUMBER_SYMMETRIC_5_ROWS: &str = "\
\x20       1\n\
\x20     2 1 2\n\
\x20   3 2 1 2 3\n\
\x20 4 3 2 1 2 3 4\n\
   5 4 3 2 1 2 3 4 5";

const _NUMBER_SQUARE_MAX_4: &str = "\
4 4 4 4 4 4 4
4 3 3 3 3 3 4
4 3 2 2 2 3 4
4 3 2 1 2 3 4
4 3 2 2 2 3 4
4 3 3 3 3 3 4
4 4 4 4 4 4 4";

const _NUMBER_SQUARE_MAX_3: &str = "\
3 3 3 3 3
3 2 2 2 3
3 2 1 2 3
3 2 2 2 3
3 3 3 3 3";

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

/// - `n` is a positive odd number, defining the number of rows in the pattern
pub fn pattern_full_diamond(n: usize) -> Option<String> {
    if n % 2 == 0 {
        return None
    }
    let row_peak = n / 2 + 1;
    let mut pattern = String::new();
    for row in 1..=n {
        let col_max = if row <= row_peak {
            row
        } else {
            n - row + 1
        };
        // Seek to correct position
        for _ in 1..=(row_peak - col_max) {
            pattern.push(' ');
        }
        for col in 1..=col_max {
            pattern.push('*');
            if col != col_max {
                pattern.push(' ');
            }
        }
        if row != n {
            pattern.push('\n');
        }
    }
    Some(pattern)
}

/// - `n` is a positive number, deifning the number of rows in the pattern
pub fn pattern_top_half_diamond_number_symmetric(n: usize) -> String {
    let mut pattern = String::new();
    let max_width = 2 * n - 1;
    for row in 1..=n {
        let width = 2 * row - 1;
        for _ in 1..=(max_width - width) {
            pattern.push(' ');
        }
        for col in 1..=width {
            let num = if col <= row {
                row - (col - 1)
            } else {
                col - (row - 1)
            };
            pattern.push_str(&num.to_string());
            if col != width {
                pattern.push(' ');
            }
        }
        if row != n {
            pattern.push('\n');
        }
    }
    pattern
}

/// - `n` is a positive number, defining the largest number in the pattern
pub fn pattern_number_square(n: usize) -> String {
    let mut pattern = String::new();
    let max_len = 2 * n - 1;
    for row in 1..=max_len {
        for col in 1..=max_len {
            let offset = row
                .min(col)
                .min(max_len + 1 - row)
                .min(max_len + 1 - col);
            let num = n - offset + 1;
            pattern.push_str(&num.to_string());
            if col != max_len {
                pattern.push(' ');
            }
        }
        if row != max_len {
            pattern.push('\n');
        }
    }
    pattern
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

    #[test]
    fn test_full_diamond() {
        assert_eq!(
            pattern_full_diamond(9),
            Some(String::from(_FULL_DIAMOND_9_ROWS))
        );
    }

    #[test]
    fn test_top_half_diamond_number_symmetric() {
        assert_eq!(
            pattern_top_half_diamond_number_symmetric(5),
            String::from(_TOP_HALF_DIAMOND_NUMBER_SYMMETRIC_5_ROWS)
        );
    }

    #[test]
    fn test_pattern_number_square() {
        assert_eq!(
            pattern_number_square(4),
            String::from(_NUMBER_SQUARE_MAX_4)
        );
        assert_eq!(
            pattern_number_square(3),
            String::from(_NUMBER_SQUARE_MAX_3)
        );
    }
}
