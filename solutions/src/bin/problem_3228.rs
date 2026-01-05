impl Solution {
    pub fn max_operations(s: String) -> i32 {
        s.into_bytes()
            .chunk_by(|a, b| a == b)
            .scan(0, |p, c| {
                if c[0] == b'1' {
                    *p += c.len() as i32;
                }
                Some(*p * (c[0] == b'0') as i32)
            })
            .sum()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_operations("1001101".to_string()));
}
