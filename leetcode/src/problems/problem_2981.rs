use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut counts = HashMap::<String, u32>::new();
        let chars = s.as_bytes();
        let (mut left, mut right) = (0, 0);
        while right <= chars.len() {
            if right < chars.len() && chars[right] == chars[left] {
                right += 1;
            } else {
                let substr = s[left..right].to_string();
                // tracing::debug!("substr: {}", substr);
                for length in 1..=substr.len() {
                    *counts.entry(substr[0..length].to_string()).or_insert(0) +=
                        (substr.len() + 1 - length) as u32;
                }
                left = right;
                if right == chars.len() {
                    break;
                }
            }
        }
        // tracing::debug!("{:?}", counts);
        counts
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .max_by_key(|(substr, _)| substr.len())
            .map_or(-1, |(substr, _)| substr.len() as i32)
    }
}
struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::maximum_length("aaaa".to_string()));
    tracing::info!("{}", Solution::maximum_length("abcaba".to_string()));
    tracing::info!("{}", Solution::maximum_length("abcdef".to_string()));
}
