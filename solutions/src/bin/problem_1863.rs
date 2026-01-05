impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |x, &y| x | y) * (1 << (nums.len() - 1))
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]))
}
