impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0; 1 + high as usize];
        for len in 0..=high as usize {
            let add_zeros = dp.get(len - zero as usize).unwrap_or(&0);
            let add_ones = dp.get(len - one as usize).unwrap_or(&0);
            let curr = (low + len as i32 <= high) as usize;
            dp[len] = (curr + add_zeros + add_ones) % 1_000_000_007
        }
        dp[high as usize] as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_good_strings(3, 3, 1, 1));
}
