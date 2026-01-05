impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter(|(_, word)| word.contains(x))
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let words = vec!["leet", "code"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    tracing::info!("{:?}", Solution::find_words_containing(words, 'e'));
}
