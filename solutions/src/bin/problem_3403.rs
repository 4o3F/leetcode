use std::collections::BinaryHeap;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        let num_friends = num_friends as usize;
        if num_friends == 1 {
            return word;
        }
        let max_substr_len = word.len() - (num_friends - 1);
        let mut substrs = Vec::<&str>::new();
        for b in 0..word.len() {
            let end_idx = *std::cmp::min(&word.len(), &(b + max_substr_len));
            if end_idx > word.len() {
                continue;
            }
            substrs.push(&word[b..end_idx]);
        }

        let mut heap: BinaryHeap<&str> = BinaryHeap::from(substrs);
        heap.pop().unwrap().to_string()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::answer_string("dbca".to_string(), 2));
}
