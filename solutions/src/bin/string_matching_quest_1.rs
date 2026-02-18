impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        'outer: for sub_length in 0..s.len() {
            if !s.len().is_multiple_of(sub_length) {
                continue;
            }
            let sub_part = &s[0..sub_length];
            for bias in 0..(s.len() / sub_length) {
                if &s[sub_length * bias..sub_length * (bias + 1)] != sub_part {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();

    tracing::info!(
        "{}",
        Solution::repeated_substring_pattern("abcabcabcabc".to_string())
    );
}
