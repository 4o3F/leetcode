use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut nums = HashSet::<i32>::from_iter(nums)
            .into_iter()
            .collect::<Vec<_>>();
        nums.sort_unstable();
        if *nums.last().unwrap() <= 0 {
            return *nums.last().unwrap();
        } else {
            return nums.iter().filter(|&x| *x >= 0).sum();
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]))
}
