impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let (mut stack, mut result) = (Vec::new(), 0);
        stack.push(nums.len() - 1);
        for (index, &num) in nums.iter().enumerate().rev() {
            if num > nums[*stack.last().unwrap()] {
                stack.push(index);
            }
        }

        for (index, &num) in nums.iter().enumerate() {
            while !stack.is_empty() && num <= nums[*stack.last().unwrap()] {
                result = result.max(stack.pop().unwrap() - index);
            }
        }
        result as i32
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let nums = vec![6, 0, 8, 2, 1, 5];
    tracing::info!("{}", Solution::max_width_ramp(nums));
    let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
    tracing::info!("{}", Solution::max_width_ramp(nums));
}
