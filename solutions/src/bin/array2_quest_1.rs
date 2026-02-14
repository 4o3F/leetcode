impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let appeared = nums
            .iter()
            .fold(vec![0u8; nums.len()], |mut appeared, num| {
                appeared[(*num - 1) as usize] += 1;
                appeared
            });
        appeared
            .iter()
            .enumerate()
            .fold(vec![0, 0], |mut result, (idx, appeared)| {
                match appeared {
                    2 => result[0] = (idx + 1) as i32,
                    0 => result[1] = (idx + 1) as i32,
                    1 => (),
                    _ => unreachable!(),
                }
                result
            })
    }
}

struct Solution;

use utils::logger::init_logger;
fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::find_error_nums(vec![1, 2, 2, 4]))
}
