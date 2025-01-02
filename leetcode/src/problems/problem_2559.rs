impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let prefix_count = words.into_iter().fold(vec![0], |mut prefix, word| {
            if (word.starts_with("a")
                || word.starts_with("e")
                || word.starts_with("i")
                || word.starts_with("o")
                || word.starts_with("u"))
                && (word.ends_with("a")
                    || word.ends_with("e")
                    || word.ends_with("i")
                    || word.ends_with("o")
                    || word.ends_with("u"))
            {
                prefix.push(prefix[prefix.len() - 1] + 1);
            } else {
                prefix.push(prefix[prefix.len() - 1]);
            }
            prefix
        });
        tracing::debug!("{:?}", prefix_count);
        queries.into_iter().fold(vec![], |mut result, query| {
            result.push(prefix_count[query[1] as usize + 1] - prefix_count[query[0] as usize]);
            result
        })
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::vowel_strings(
            vec![
                "aba".to_string(),
                "bcb".to_string(),
                "ece".to_string(),
                "aa".to_string(),
                "e".to_string()
            ],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]],
        )
    );
}
