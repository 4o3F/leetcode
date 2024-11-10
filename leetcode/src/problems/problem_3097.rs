use std::i32;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 1;
        }
        let (mut left_idx, mut current_or, mut min_length) = (0, 0, i32::MAX);
        let mut or_count = [0; 32];
        for (right_idx, &num) in (1..).zip(nums.iter()) {
            if num >= k {
                return 1;
            }
            current_or |= num;
            (0..32).for_each(|idx| {
                or_count[idx] += ((num >> idx) & 1 == 1) as i32;
            });
            while current_or >= k {
                min_length = min_length.min(right_idx - left_idx);
                (0..32).for_each(|idx| {
                    let to_remove = nums[left_idx as usize];
                    let decrement = to_remove >> idx & 1 == 1;
                    if decrement && or_count[idx] == 1 {
                        current_or ^= 1 << idx;
                    }
                    or_count[idx] -= decrement as i32;
                });
                left_idx += 1;
            }
        }
        match min_length {
            i32::MAX => -1,
            _ => min_length,
        }
    }
}

struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::minimum_subarray_length(vec![2, 25, 32, 1], 59)
    );
    tracing::info!("{}", Solution::minimum_subarray_length(vec![1, 2], 0));
}
