impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.push(0);
        let mut result = 0;
        let mut current_count = 0;
        for num in nums {
            if num == 1 {
                current_count += 1;
            } else {
                result = current_count.max(result);
                current_count = 0;
            }
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();

    tracing::info!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
    tracing::info!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
    );
}
