impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a < b)
            .map(|v| v.iter().sum())
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50])
    );
}
