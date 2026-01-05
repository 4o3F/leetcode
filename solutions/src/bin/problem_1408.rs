impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_by_key(|word| word.len());
        let mut result = vec![];
        for (idx_start, word) in words.iter().enumerate() {
            for idx in idx_start + 1..words.len() {
                if words.get(idx).unwrap().contains(word) {
                    result.push(word.clone());
                    break;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let input = vec!["leetcoder", "leetcode", "od", "hamlet", "am"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    tracing::info!("{:?}", Solution::string_matching(input))
}
