impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        nums.iter()
            .fold(Vec::<i32>::with_capacity(nums.len()), |mut result, num| {
                let idx = sorted_nums.partition_point(|x| x < num);
                result.push(idx as i32);
                result
            })
    }
}

struct Solution;
use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
    );
}
