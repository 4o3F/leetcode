impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut best_diff = i32::MAX;

        for idx in 0..nums.len() {
            let mut left = idx + 1;
            let mut right = nums.len() - 1;
            let pre_diff = nums[idx] - target;
            while left < right {
                let diff = pre_diff + nums[left] + nums[right];
                if diff.abs() < best_diff.abs() {
                    best_diff = diff;
                }
                match diff.signum() {
                    1 => right -= 1,
                    -1 => left += 1,
                    0 => return target,
                    _ => unreachable!(),
                }
            }
        }
        target + best_diff
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
}
