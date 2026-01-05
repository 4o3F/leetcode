impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        s.bytes()
            .fold(vec![0; 26], |mut freq, b| {
                freq[(b - b'a') as usize] += 1;
                freq
            })
            .iter()
            .enumerate()
            .filter(|(_, &f)| f % 2 == 1)
            .count()
            <= k as usize
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::can_construct("annabelle".to_string(), 2))
}
