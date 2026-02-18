use std::collections::VecDeque;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut s = VecDeque::from_iter(s.chars());
        let mut count = 0;
        while s.iter().collect::<String>() != goal && count < s.len() {
            let c = s.pop_front().unwrap();
            s.push_back(c);
            count += 1;
        }
        count < s.len()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string())
    );
}
