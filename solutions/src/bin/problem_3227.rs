impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars().any(|c| (0x208222 >> ((c as u32) & 31) & 1) != 0)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::does_alice_win("leetcoder".to_string()))
}
