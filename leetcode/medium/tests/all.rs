use leetcode_medium::dynamic;

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
}
