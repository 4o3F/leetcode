impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![vec![0; 2001]; nums.len()];
        dp[0][1000 + nums[0] as usize] = 1;
        dp[0][1000 - nums[0] as usize] += 1;
        for i in 1..dp.len() {
            for target in 0..2001 {
                if dp[i - 1][target] > 0 {
                    dp[i][target + nums[i] as usize] += dp[i - 1][target];
                    dp[i][target - nums[i] as usize] += dp[i - 1][target]
                }
            }
        }
        dp[dp.len() - 1][1000 + target as usize]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}
