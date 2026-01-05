use std::collections::VecDeque;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let (mut iq, mut dq, mut j) = (VecDeque::new(), VecDeque::new(), 0);
        nums.iter()
            .enumerate()
            .map(|(i, &n)| {
                while iq.back().is_some_and(|&b| nums[b] >= n) {
                    iq.pop_back();
                }
                while dq.back().is_some_and(|&b| nums[b] <= n) {
                    dq.pop_back();
                }
                iq.push_back(i);
                dq.push_back(i);
                while n - nums[*iq.front().unwrap()] > 2 {
                    j = iq.pop_front().unwrap() + 1;
                }
                while nums[*dq.front().unwrap()] - n > 2 {
                    j = dq.pop_front().unwrap() + 1;
                }
                1 + i as i64 - j as i64
            })
            .sum()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::continuous_subarrays(vec![5, 4, 2, 4]));
}
