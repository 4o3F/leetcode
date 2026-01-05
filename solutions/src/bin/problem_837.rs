impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || n >= k + max_pts - 1 {
            return 1.0;
        }
        let (n, k, m) = (n as usize, k as usize, max_pts as usize);
        let mut dp = Vec::with_capacity(n + 1);
        dp.push(1.0);
        let mut w = 1.0;
        let mut res = 0.0;
        for i in 1..=n {
            dp.push(w / m as f64);
            if i < k {
                w += dp[i]
            } else {
                res += dp[i]
            }
            if i >= m {
                w -= dp[i - m]
            }
        }
        res
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::new21_game(10, 1, 10))
}
