use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let distinct_count = HashSet::<i32>::from_iter(nums.clone()).len();
        let (mut left_idx, mut right_idx) = (0, 0);
        let mut current_nums = HashMap::<i32, i32>::new();

        let mut count = 0;
        while right_idx < nums.len() {
            {
                let entry = current_nums.entry(nums[right_idx]).or_insert(0);
                *entry += 1;
            }

            while current_nums.get(&nums[left_idx]).unwrap() > &1 {
                let entry = current_nums.get_mut(&nums[left_idx]).unwrap();
                *entry -= 1;
                left_idx += 1;
            }

            if current_nums.len() == distinct_count {
                count += left_idx + 1;
            }

            right_idx += 1;
        }

        count as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2])
    )
}
