use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let (dominant, freq) = nums
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut freq, num| {
                freq.entry(*num).and_modify(|x| *x += 1).or_insert(1);
                freq
            })
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        let mut dominant_count = 0;
        for i in 0..nums.len() {
            if nums[i] == dominant {
                dominant_count += 1;
            }
            if dominant_count > (i + 1) as i32 / 2
                && (freq - dominant_count) > (nums.len() - i - 1) as i32 / 2
            {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::minimum_index(vec![1, 2, 2, 2]));
}
