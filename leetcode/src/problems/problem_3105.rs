impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a > b)
            .chain(nums.chunk_by(|a, b| a < b))
            .map(|c| c.len() as i32)
            .max()
            .unwrap_or(1)
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2])
    );
}
