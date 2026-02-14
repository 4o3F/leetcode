impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let appeared = nums
            .iter()
            .fold(vec![false; nums.len()], |mut appeared, num| {
                appeared[(*num - 1) as usize] = true;
                appeared
            });
        (1..=nums.len()).fold(Vec::new(), |mut result, num| {
            if !appeared[num - 1] {
                result.push(num as i32);
            }
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
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}
