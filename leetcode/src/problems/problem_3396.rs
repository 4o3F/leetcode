use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut last_idx: i32 = -1;
        let mut exist = HashSet::<i32>::new();
        for (idx, num) in nums.iter().enumerate().rev() {
            if exist.contains(num) {
                last_idx = idx as i32;
                break;
            } else {
                exist.insert(*num);
            }
        }
        ((last_idx + 3) / 3) as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::minimum_operations(vec![1, 2, 2, 3, 3, 5, 7])
    );
    tracing::info!(
        "{}",
        Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7])
    );
    tracing::info!("{}", Solution::minimum_operations(vec![4, 5, 6, 4, 4]));
    tracing::info!("{}", Solution::minimum_operations(vec![6, 7, 8, 9]));
}
