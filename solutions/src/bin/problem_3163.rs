impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut current_char = word.chars().nth(0).unwrap();
        let mut current_count = 1;
        let mut result = String::new();
        for c in word.chars().skip(1) {
            // tracing::debug!("{} current_char {} current_count {}", c, current_char, current_count);
            if c == current_char && current_count < 9 {
                current_count += 1;
            } else {
                result.push_str(current_count.to_string().as_str());
                result.push(current_char);
                current_char = c;
                current_count = 1;
                // tracing::debug!("{}", result);
            }
        }
        result.push_str(current_count.to_string().as_str());
        result.push(current_char);
        result
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::compressed_string("abcde".to_string()));
}
