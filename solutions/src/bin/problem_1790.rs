impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut diff = Vec::new();
        s1.bytes()
            .enumerate()
            .zip(s2.bytes())
            .for_each(|((idx, a), b)| {
                if a != b {
                    diff.push((idx, a, b));
                }
            });
        // tracing::info!("{:?}", diff);
        if diff.is_empty() {
            true
        } else if diff.len() != 2 {
            false
        } else {
            diff[0].1 == diff[1].2 && diff[0].2 == diff[1].1
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::are_almost_equal("bank".to_string(), "kanb".to_string())
    );
    tracing::info!(
        "{}",
        Solution::are_almost_equal("kelb".to_string(), "kelb".to_string())
    );
}
