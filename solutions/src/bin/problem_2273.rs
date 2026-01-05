impl Solution {
    pub fn remove_anagrams(mut words: Vec<String>) -> Vec<String> {
        words.dedup_by_key(|k| {
            k.bytes().fold([0; 26], |mut f, b| {
                f[(b - b'a') as usize] += 1;
                f
            })
        });
        words
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::remove_anagrams(
            vec!["abba", "baba", "bbaa", "cd", "cd"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    )
}
