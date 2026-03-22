use standard_impl::searching::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_none_if_not_found() {
        let vec1: Vec<i32> = vec![
            10, 12, 15, 20, 32, 40, 50, 70, 71,
            71, 72, 73, 80, 80, 80, 90, 100,
        ];
        assert_eq!(linear_search(&vec1, 24), None,
                   "Reported some index for 24 in {:?}", vec1);
        assert_eq!(binary_search(&vec1, 24), None,
                   "Reported some index for 24 in {:?}", vec1);
    }

    #[test]
    fn return_some_if_found() {
        let vec1: Vec<i32> = vec![
            10, 12, 15, 20, 32, 40, 50, 70, 71,
            71, 72, 73, 80, 80, 80, 90, 100,
        ];
        let expected_results: Vec<(i32, usize)> = vec![
            (71, 8),
            (10, 0),
            (100, 16),
        ];
        for pair in expected_results {
            let Some(found_at_linear) = linear_search(&vec1, pair.0) else {
                panic!("Target not found in sequence (linear search)");
            };
            assert_eq!(found_at_linear, pair.1,
                       "Reported wrong index ({found_at_linear}) for {} in {:?} (expected: {})",
                       pair.0, vec1, pair.1);
            let Some(found_at_binary) = binary_search(&vec1, pair.0) else {
                panic!("Target not found in sequence (binary search)");
            };
            assert_eq!(found_at_binary, pair.1,
                       "Reported wrong index ({found_at_binary}) for {} in {:?} (expected: {})",
                       pair.0, vec1, pair.1);
        }
    }

    #[test]
    fn works_for_empty_sequence() {
        assert_eq!(linear_search(&[], 10), None, "Somehow, a target was found");
        assert_eq!(binary_search(&[], 10), None, "Somehow, a target was found");
    }
    
}
