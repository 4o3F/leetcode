impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        (0..=n.min(limit))
            .map(|i| 0.max(limit.min(n - i) - 0.max(n - i - limit) + 1) as i64)
            .sum()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::distribute_candies(5, 2))
}
