use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let freq = s.chars().fold(HashMap::<char, i32>::new(), |mut freq, c| {
            let entry = freq.entry(c).or_insert(0);
            *entry += 1;
            freq
        });
        let (mut max_odd, mut min_even) = (0, i32::MAX);
        for freq in freq.values() {
            if freq % 2 == 0 {
                min_even = min_even.min(*freq);
            } else {
                max_odd = max_odd.max(*freq);
            }
        }
        max_odd - min_even
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_difference("aaaaabbc".to_string()));
}
