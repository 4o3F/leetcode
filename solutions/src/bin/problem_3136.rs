impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut vowel_flag = false;
        let mut consont_flag = false;

        for ch in word.chars() {
            if ch.is_ascii_digit() {
                continue;
            }

            match ch {
                'a' | 'A' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' | 'e' | 'E' => vowel_flag = true,
                _ => {
                    if ch.is_ascii_lowercase() || ch.is_ascii_uppercase() {
                        consont_flag = true;
                    } else {
                        return false;
                    }
                }
            }
        }

        word.len() >= 3 && vowel_flag && consont_flag
    }
}
struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::is_valid("234Adas".to_string()))
}
