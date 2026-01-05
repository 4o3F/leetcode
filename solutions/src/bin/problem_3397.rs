impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut prev_assigned: i64 = -9_000_000_000_000_000;
        let mut distinct_count: i32 = 0;
        for &v in nums.iter() {
            let num = v as i64;
            let assigned = std::cmp::max(num - k as i64, prev_assigned + 1);
            if assigned <= num + k as i64 {
                distinct_count += 1;
                prev_assigned = assigned;
            }
        }
        distinct_count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2)
    )
}
