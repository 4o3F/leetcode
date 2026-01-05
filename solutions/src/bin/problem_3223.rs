impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold(vec![0; 26], |mut freq, b| {
                freq[(b - b'a') as usize] += 1;
                freq
            })
            .into_iter()
            .map(|freq| {
                // tracing::info!("{}", freq);
                if freq > 2 {
                    if freq % 2 == 0 {
                        2
                    } else {
                        1
                    }
                } else {
                    freq
                }
            })
            .sum()
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::minimum_length("abaacbcbb".to_string()))
}
