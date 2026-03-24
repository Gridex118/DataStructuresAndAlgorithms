use standard_impl::sorting::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_empty_array() {
        let mut vec: Vec<i32> = vec![];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![],
                   "Somehow failed to sort an empty array");
    }

    #[test]
    fn sorted_array_remains_unaffected() {
        for vec_base in [
            vec![ 2, 10, 16, 19, 50, 78 ],
            vec![ 2 ],
        ] {
            let mut vec = vec_base.to_vec();
            bubble_sort(&mut vec);
            assert_eq!(vec, vec_base,
                       "Somehow a sorted array was altered");
        }
    }

    #[test]
    fn successfully_sorts_unsorted_array() {
        let mut vec = vec![ 10, 13, 6, 1, 50, 23, 19, 45, 50 ];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![ 1, 6, 10, 13, 19, 23, 45, 50, 50 ],
                   "Failed to sort array");
    }

}
