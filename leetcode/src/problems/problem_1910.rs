use std::collections::VecDeque;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = VecDeque::new();
        let s = s.as_bytes();
        let part = part.as_bytes();
        for i in 0..s.len() {
            stack.push_back(s[i]);
            while !stack.is_empty() && stack.len() >= part.len() {
                let mut j = 0;
                for k in 0..part.len() {
                    if stack[stack.len() - 1 - k] != part[part.len() - 1 - k] {
                        break;
                    }
                    j += 1;
                }
                if j == part.len() {
                    for _ in 0..j {
                        stack.pop_back();
                    }
                } else {
                    break;
                }
            }
        }
        let mut res = String::new();
        for i in 0..stack.len() {
            res.push(stack[i] as char);
        }
        res
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string())
    );
    tracing::info!(
        "{}",
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
    );
}
