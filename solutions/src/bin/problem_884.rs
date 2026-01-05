struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split_whitespace()
            .chain(s2.split_whitespace())
            .fold(HashMap::new(), |mut word_counter, word| {
                *word_counter.entry(word.to_string()).or_insert(0) += 1;
                word_counter
            })
            .into_iter()
            .filter(|item| item.1 == 1)
            .map(|item| item.0)
            .collect::<Vec<String>>()
    }
}
fn main() {
    use utils::prelude::*;
    init_logger();
    let s1 = "s z z z s".to_string();
    let s2 = "s z ejt".to_string();
    tracing::info!("{:?}", Solution::uncommon_from_sentences(s1, s2));
}
