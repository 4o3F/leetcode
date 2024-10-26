use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let (mut result, mut unique_substr) = (0, HashSet::<String>::new());

        fn dfs(s: &str, result: &mut i32, unique_substr: &mut HashSet<String>) {
            *result = unique_substr.len().max(*result as usize) as i32;
            for j in 0..s.len() {
                if unique_substr.insert(s[..=j].to_string()) {
                    dfs(&s[j + 1..], result, unique_substr);
                    unique_substr.remove(&s[..=j]);
                }
            }
        }

        dfs(s.as_str(), &mut result, &mut unique_substr);
        result
    }
}

struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::max_unique_split("pineapplepenapple".to_string())
    );
}
