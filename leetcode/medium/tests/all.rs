use leetcode_medium::dynamic;
use leetcode_medium::search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn house_robber_two() {
        for (nums, max_money) in [
            (vec![2, 3, 2], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![1, 2, 3], 3),
        ] {
            assert_eq!(max_money, dynamic::rob_house_two(nums.to_vec()),
                       "Failed for {nums:?}");
        }
    }

    #[test]
    fn two_sum_two_array_is_sorted() {
        for (numbers, target, expected) in [
            (vec![2, 7, 11, 15], 9, vec![1, 2]),
            (vec![2, 3, 4], 6, vec![1, 3]),
            (vec![-1, 0], -1, vec![1, 2]),
            (vec![1, 3, 4, 4, 5, 9, 8, 10, 13], 6, vec![1, 5]),
        ] {
            assert_eq!(expected, search::two_sum_sorted(numbers.to_vec(), target),
                       "Failed for {numbers:?} : {target}");
        }
    }
}
