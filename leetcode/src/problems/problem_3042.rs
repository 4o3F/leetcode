impl Solution {
    fn is_prefix_and_suffix(word1: &String, word2: &String) -> bool {
        if word2.starts_with(word1) && word2.ends_with(word1) {
            true
        } else {
            false
        }
    }
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if Self::is_prefix_and_suffix(&words[i], &words[j]) {
                    // tracing::debug!("{} {}", &words[i], &words[j]);
                    result += 1;
                }
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    let input = vec!["a", "aba", "ababa", "aa"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    tracing::info!("{}", Solution::count_prefix_suffix_pairs(input))
}
