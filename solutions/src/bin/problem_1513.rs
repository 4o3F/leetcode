impl Solution {
    pub fn num_sub(s: String) -> i32 {
        s.into_bytes()
            .chunk_by(|a, b| a == b)
            .filter_map(|c| (c[0] == b'1').then_some(c.len()))
            .fold(0, |acc, l| {
                (((acc << 1) + (l * (l + 1))) >> 1) % 1_000_000_007
            }) as _
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::num_sub("00011".to_string()));
}
