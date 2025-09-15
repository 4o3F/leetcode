impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        text.split(' ')
            .map(|x| x.to_string())
            .filter(|word| {
                let chars = word.chars().collect::<Vec<_>>();
                for c in broken_letters.chars() {
                    if chars.contains(&c) {
                        return false;
                    }
                }
                true
            })
            .count() as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::can_be_typed_words("hello world".to_string(), "ad".to_string())
    )
}
