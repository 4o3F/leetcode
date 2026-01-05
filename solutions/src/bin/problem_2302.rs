impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut count = 0i64;
        let mut left_idx = 0;
        let mut current_sum = 0;
        for (right_idx, &num) in nums.iter().enumerate() {
            current_sum += i64::from(num);
            while left_idx <= right_idx {
                let score = current_sum * (right_idx - left_idx + 1) as i64;
                // tracing::info!(
                //     "left_idx {} right_idx {} score {} current_sum {}",
                //     left_idx,
                //     right_idx,
                //     score,
                //     current_sum
                // );
                if score < k {
                    count += (right_idx - left_idx + 1) as i64;
                    break;
                } else {
                    // Do not match requirement, moving left_idx
                    current_sum -= i64::from(nums[left_idx]);
                    left_idx += 1;
                }
            }
            // tracing::info!(
            //     "count {} left_idx {} right_idx {}",
            //     count,
            //     left_idx,
            //     right_idx
            // );
        }
        count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_subarrays(vec![5, 2, 6, 8, 9, 7], 50));
    tracing::info!("{}", Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10));
    tracing::info!("{}", Solution::count_subarrays(vec![1, 1, 1], 5));
}
