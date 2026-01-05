impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .filter(|nums| (nums[0] + nums[2]) * 2 == nums[1])
            .count() as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_subarrays(vec![1, 2, 1, 4, 1]));
}
