impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let old_min = nums.first().unwrap();
        let old_max = nums.last().unwrap();

        nums.windows(2)
            .map(|window| {
                let new_max = (window[0] + k).max(old_max - k);
                let new_min = (window[1] - k).min(old_min + k);

                new_max - new_min
            })
            .min()
            .unwrap_or(0)
            .min(old_max - old_min)
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::smallest_range_ii(vec![1, 3, 6], 3));
}
