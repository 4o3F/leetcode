use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut nums = HashSet::<i32>::from_iter(nums)
            .into_iter()
            .collect::<Vec<_>>();
        nums.sort_unstable();
        if *nums.last().unwrap() <= 0 {
            *nums.last().unwrap()
        } else {
            nums.iter().filter(|&x| *x >= 0).sum()
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]))
}
