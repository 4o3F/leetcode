impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut dp: Vec<Vec<&str>> = vec![vec![]; words.len()];
        dp[words.len() - 1] = vec![];
        let (mut max, mut max_idx) = (0, 0);
        for idx in (0..words.len() - 1).rev() {
            for idx1 in idx + 1..words.len() {
                if Solution::is_match(&words, &groups, idx, idx1) && dp[idx].len() <= dp[idx1].len()
                {
                    let mut items = vec![words[idx1].as_str()];
                    for item in dp[idx1].iter() {
                        items.push(item);
                    }
                    dp[idx] = items;
                    if dp[idx].len() > max {
                        max = dp[idx].len();
                        max_idx = idx;
                    }
                }
            }
        }
        let mut result = vec![words[max_idx].clone()];
        for item in dp[max_idx].iter() {
            result.push(item.to_string())
        }
        result
    }

    pub fn is_match(words: &Vec<String>, groups: &Vec<i32>, idx1: usize, idx2: usize) -> bool {
        if groups[idx1] == groups[idx2] {
            return false;
        }
        if words[idx1].len() != words[idx2].len() {
            return false;
        }
        let mut dist = 0;
        let word1 = words[idx1].as_bytes();
        let word2 = words[idx2].as_bytes();
        for idx in 0..word1.len() {
            if word1[idx] != word2[idx] {
                dist += 1;
            }
        }
        dist == 1
    }
}

struct Solution;

pub fn run() {
    let words = vec!["bab", "dab", "cab"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    tracing::info!(
        "{:?}",
        Solution::get_words_in_longest_subsequence(words, vec![1, 2, 2])
    );
}
