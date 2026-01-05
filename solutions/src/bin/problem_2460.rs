impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let zero_count = nums.iter().filter(|&&x| x == 0).count();
        nums.retain(|&x| x != 0);
        nums.resize(nums.len() + zero_count, 0);
        nums
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]));
}
