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
}
