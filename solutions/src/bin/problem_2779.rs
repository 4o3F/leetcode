impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        for (index, num) in nums.iter().enumerate() {
            let point = nums.partition_point(|n| n - num <= 2 * k);
            result = result.max(point - index);
        }
        result as i32
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_beauty(vec![4, 6, 1, 2], 2));
    tracing::info!("{}", Solution::maximum_beauty(vec![1, 1, 1, 1], 10));
    tracing::info!("{}", Solution::maximum_beauty(vec![49, 26], 12));
}
