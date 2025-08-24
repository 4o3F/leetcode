impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut last_zero_idx: Option<usize> = None;
        let mut last_start_idx = 0;
        let mut max = 0;
        for (idx, &num) in nums.iter().enumerate() {
            if num == 0 {
                if let Some(last_zero_idx) = last_zero_idx {
                    max = max.max(idx - last_start_idx - 1);
                    last_start_idx = last_zero_idx + 1;
                }
                last_zero_idx = Some(idx);
            }
        }
        match last_zero_idx {
            Some(_) => max.max(nums.len() - last_start_idx - 1) as i32,
            None => (nums.len() - 1) as i32,
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::longest_subarray(vec![1, 1, 1]));
}
