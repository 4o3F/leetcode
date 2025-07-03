impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        word.chars()
            .collect::<Vec<_>>()
            .chunk_by(|a, b| a == b)
            .map(|x| x.len() as i32 - 1)
            .sum::<i32>()
            + 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::possible_string_count("abbcccc".to_string()))
}
