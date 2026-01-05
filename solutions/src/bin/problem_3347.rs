impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort_unstable();

        let len = nums.len();

        let mut max_freq = 0_usize;
        let num_operations_usize = num_operations as usize;
        let mut left;
        let mut right = 0_usize;
        let mut left_single = 0_usize;
        let mut right_single = 0_usize;
        let mut left_doubled = 0_usize;
        let mut right_doubled = 0_usize;

        while right < len {
            // First two-pointers logic
            left = right;
            let curr = nums[left];

            while right < len && nums[right] == curr {
                right += 1;
            }

            // Second two-pointers logic
            let min_single = curr - k;
            let max_single = curr + k;

            while nums[left_single] < min_single {
                left_single += 1;
            }

            while right_single < len && nums[right_single] <= max_single {
                right_single += 1;
            }

            let total_free_cost = right - left;
            let total_with_cost = right_single - right + left - left_single;
            let curr_freq = total_free_cost + total_with_cost.min(num_operations_usize);
            max_freq = max_freq.max(curr_freq);

            // Third two-pointers logic
            let min_doubled = (curr as i64 - (2 * k) as i64).max(i32::MIN as i64) as i32;
            let max_doubled = (curr as i64 + (2 * k) as i64).min(i32::MAX as i64) as i32;

            while nums[left_doubled] < min_doubled {
                left_doubled += 1;
            }

            while right_doubled < len && nums[right_doubled] <= max_doubled {
                right_doubled += 1;
            }

            // Assume that all numbers in this range need operation
            // as an example [1,2,3] k = 1, op = 2
            // then while this conditions cant set max_freq higher than 2,
            // but number "2" can check it when it is candidate with prev conditions as 2 + k, 2 - k
            let total_double_on_left = right - left_doubled;
            max_freq = max_freq.max(total_double_on_left.min(num_operations_usize));

            let total_double_on_right = right_doubled - left;
            max_freq = max_freq.max(total_double_on_right.min(num_operations_usize));
        }

        max_freq as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_frequency(vec![1, 4, 5], 1, 2))
}
