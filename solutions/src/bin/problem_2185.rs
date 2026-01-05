impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().fold(0, |acc, word| {
            if word.starts_with(&pref) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let words = vec!["pay", "attention", "practice", "attend"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    tracing::info!("{}", Solution::prefix_count(words, "at".to_string()))
}
