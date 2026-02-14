impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0i32; nums.len()];
        for (idx, num) in nums.iter().enumerate() {
            let new_idx = if idx < n { idx * 2 } else { (idx - n) * 2 + 1 };
            result[new_idx] = *num;
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3));
    tracing::info!("{:?}", Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4));
}
