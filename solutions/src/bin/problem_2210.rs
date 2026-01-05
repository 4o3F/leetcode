use itertools::Itertools;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        nums.iter()
            .dedup()
            .tuple_windows::<(_, _, _)>()
            .filter(|(a, b, c)| (a > b) == (b < c))
            .count() as _
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_hill_valley(vec![1, 1, 2, 2, 2, 2, 2, 1])
    )
}
