impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for &idx in nums.iter() {
            result.push(nums[idx as usize]);
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::build_array(vec![0, 2, 1, 5, 3, 4]));
}
