impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() - 1)
            .map(|idx| {
                let high = nums[idx + 1..].partition_point(|&x| nums[idx] + x <= upper);
                if high == 0 {
                    return 0;
                }
                let low = nums[idx + 1..].partition_point(|&x| nums[idx] + x < lower);
                (high - low) as i64
            })
            .sum()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6)
    );
}
