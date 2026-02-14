impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        result.extend(nums);
        result
    }
}

struct Solution;
use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::get_concatenation(vec![1, 3, 2, 1]))
}
