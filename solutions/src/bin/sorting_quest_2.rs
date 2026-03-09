impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut ans = 0;
        let mut ops = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                ops += 1;
            }
            ans += ops;
        }

        ans
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::reduction_operations(vec![1, 1, 2, 2, 3]))
}
