impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if !s.len().is_multiple_of(2) {
            return false;
        }
        let (mut b, o, s) = ([0, 0], [b'(', b')'], s.as_bytes());
        for i in 0..s.len() {
            for j in 0..2 {
                let i = if j > 0 { s.len() - 1 - i } else { i };
                if s[i] == o[j] || locked.as_bytes()[i] == b'0' {
                    b[j] += 1;
                } else {
                    b[j] -= 1;
                    if b[j] < 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::can_be_valid("))()))".to_string(), "010100".to_string())
    )
}
