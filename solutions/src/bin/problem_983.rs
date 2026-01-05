impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; days.len() + 1];
        dp[0] = 0;
        for start in 0..days.len() {
            let mut i = start;
            for (c, d) in costs.iter().zip([1, 7, 30]) {
                while i < days.len() && days[i] - days[start] < d {
                    i += 1
                }
                dp[i] = dp[i].min(dp[start] + c)
            }
        }
        dp[days.len()]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
    );
}
