impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words = sentence.split(' ').collect::<Vec<_>>();
        for (index, word) in words.iter().enumerate() {
            if word.starts_with(search_word.as_str()) {
                return index as i32 + 1;
            }
        }
        -1
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string())
    );
}
