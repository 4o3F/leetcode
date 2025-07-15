impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut vowel_flag = false;
        let mut consont_flag = false;

        for ch in word.chars() {
            if ch >= '0' && ch <= '9' {
                continue;
            }

            match ch {
                'a' | 'A' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' | 'e' | 'E' => vowel_flag = true,
                _ => {
                    if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') {
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

pub fn run() {
    tracing::info!("{}", Solution::is_valid("234Adas".to_string()))
}
