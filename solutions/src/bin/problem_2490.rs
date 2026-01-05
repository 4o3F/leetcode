impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.trim();
        if sentence.len() < 2 {
            return true;
        } else if sentence.chars().nth(0) != sentence.chars().nth(sentence.len() - 1) {
            // tracing::debug!("First & last char don't match");
            return false;
        }
        let words = sentence.split(' ').collect::<Vec<_>>();
        for words in words.windows(2) {
            assert!(words.len() == 2);
            if words[0].chars().nth(words[0].len() - 1) != words[1].chars().nth(0) {
                // tracing::debug!("{} {}", words[0], words[1]);
                return false;
            }
        }
        true
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::is_circular_sentence("leetcode exercises sound delightful".to_string())
    );
    tracing::info!("{}", Solution::is_circular_sentence("eetcode".to_string()));
    tracing::info!(
        "{}",
        Solution::is_circular_sentence("Leetcode is cool".to_string())
    );
}
