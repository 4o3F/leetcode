impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_res = nums[0];
        let mut max_ending = nums[0];
        let mut min_res = nums[0];
        let mut min_ending = nums[0];
        for num in nums.iter().skip(1) {
            max_ending = *num.max(&(max_ending + num));
            min_ending = *num.min(&(min_ending + num));
            max_res = max_res.max(max_ending);
            min_res = min_res.min(min_ending);
        }
        max_res.abs().max(min_res.abs())
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_absolute_sum([1, -3, 2, 3, -4].to_vec()))
}
