use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let frequencies = nums
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut freq, num| {
                let entry = freq.entry(*num).or_insert(0);
                *entry += 1;
                freq
            });
        frequencies.keys().fold(0, |max_length, key| {
            if !frequencies.contains_key(&(*key - 1)) && !frequencies.contains_key(&(*key + 1)) {
                return max_length;
            }

            let current_freq = frequencies.get(key).unwrap();
            let current_max = current_freq
                + frequencies
                    .get(&(*key - 1))
                    .unwrap_or(&0)
                    .max(frequencies.get(&(*key + 1)).unwrap_or(&0));
            max_length.max(current_max)
        })
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
}
