impl Solution {
    pub fn zero_filled_subarray(mut nums: Vec<i32>) -> i64 {
        nums.push(1);
        let mut start_idx: Option<usize> = None;
        let mut result = 0;
        for (end_idx, &num) in nums.iter().enumerate() {
            if num == 0 {
                if start_idx.is_none() {
                    start_idx = Some(end_idx)
                }
            } else if let Some(start_idx_tmp) = start_idx {
                let tmp = (end_idx - start_idx_tmp) as i64;
                result += (1 + tmp) * tmp / 2;
                start_idx = None;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]))
}
