impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut pointer = 0usize;
        let mut result = 0;
        let mut current_repeat_time = 0;

        if word.is_empty() || sequence.len() < word.len() {
            return 0;
        }

        while pointer <= sequence.len() - word.len() {
            let matched = sequence[pointer..pointer + word.len()] == word;
            // tracing::info!("pointer {} matched {}", pointer, matched);
            if matched {
                current_repeat_time += 1;
                pointer += word.len();
            } else {
                result = result.max(current_repeat_time);
                pointer = pointer - (current_repeat_time as usize * word.len()) + 1;
                current_repeat_time = 0;
            }
        }
        result.max(current_repeat_time)
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_repeating("a".to_string(), "aa".to_string())
    );
}
