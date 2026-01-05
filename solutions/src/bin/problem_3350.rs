use itertools::Itertools;
use std::iter::once;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        once(0)
            .chain(nums.chunk_by(|a, b| a < b).map(|c| c.len()))
            .tuple_windows()
            .map(|(ck1, ck2)| ck1.min(ck2).max(ck2 / 2))
            .max()
            .unwrap() as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1])
    )
}
