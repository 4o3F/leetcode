use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let total_pairs = nums.len() * (nums.len() - 1) / 2;
        total_pairs as i64
            - nums
                .into_iter()
                .enumerate()
                .fold(HashMap::<i32, i64>::new(), |mut f, (idx, num)| {
                    f.entry(num - idx as i32)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                    f
                })
                .values()
                .fold(0, |mut sum, &n| {
                    sum += n * (n - 1) / 2;
                    sum
                })
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_bad_pairs(vec![4, 1, 3, 3]));
}
