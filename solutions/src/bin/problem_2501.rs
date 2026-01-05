use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i64> = HashSet::from_iter(nums.into_iter().map(i64::from));
        let mut max = -1;
        for &num in &nums_set {
            let mut num = num;
            let mut current_max = 1;
            while nums_set.contains(&(num * num)) {
                num = num * num;
                current_max += 1;
            }
            if current_max >= 2 {
                max = max.max(current_max);
            }
        }
        max
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2])
    );
}
