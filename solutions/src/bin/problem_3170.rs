use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut h: BinaryHeap<(Reverse<char>, usize)> = BinaryHeap::new();
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                h.pop();
            } else {
                h.push((Reverse(c), i));
            }
        }

        if h.is_empty() {
            return String::from("");
        }

        let max = h.iter().map(|(_, i)| *i).max().unwrap_or(0);

        let mut x = vec!['\0'; max + 1];

        while let Some((Reverse(c), i)) = h.pop() {
            x[i] = c;
        }

        let result: String = x.iter().filter(|&&c| c != '\0').collect();
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::clear_stars("aaba*".to_string()))
}
