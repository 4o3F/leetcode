impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total = nums.iter().sum::<i32>();
        if total % 2 == 1 {
            return false;
        }
        let partition_sum = (total / 2) as usize;
        let n = nums.len();
        let mut dp = vec![vec![false; partition_sum + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = true;
        }
        for i in 1..=n {
            for j in 1..=partition_sum {
                if nums[i - 1] > j as i32 {
                    dp[i][j] = dp[i - 1][j]
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize]
                }
            }
        }

        dp[n][partition_sum]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::can_partition(vec![1, 5, 11, 5]));
}
