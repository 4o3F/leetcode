impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::<Vec<i32>>::new();
        for group in nums.windows(3).step_by(3) {
            // tracing::info!("{:?}", group);
            if group[2] - group[0] > k {
                return Vec::new();
            }
            result.push(group.to_vec());
        }

        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2)
    )
}
