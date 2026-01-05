impl Solution {
    pub fn max_adjacent_distance(mut nums: Vec<i32>) -> i32 {
        nums.push(*nums.first().unwrap());
        // tracing::info!("{:?}", nums);
        nums.windows(2)
            .map(|nums| nums[1].abs_diff(nums[0]) as i32)
            .max()
            .unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_adjacent_distance(vec![1, 2, 4]))
}
