use std::collections::VecDeque;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut queue1: VecDeque<&str> = sentence1.split_whitespace().collect();
        let mut queue2: VecDeque<&str> = sentence2.split_whitespace().collect();

        while !queue1.is_empty()
            && !queue2.is_empty()
            && queue1.front().unwrap() == queue2.front().unwrap()
        {
            queue1.pop_front();
            queue2.pop_front();
        }

        while !queue1.is_empty()
            && !queue2.is_empty()
            && queue1.back().unwrap() == queue2.back().unwrap()
        {
            queue1.pop_back();
            queue2.pop_back();
        }
        queue1.is_empty() || queue2.is_empty()
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let sentence1 = "My name is Haley".to_string();
    let sentence2 = "My Haley".to_string();
    tracing::info!(
        "{:?}",
        Solution::are_sentences_similar(sentence1, sentence2)
    );
}
