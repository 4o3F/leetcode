impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let mut result = i32::MAX;
        for block in blocks.windows(k as usize) {
            let mut count = 0;
            for i in 0..k {
                if block[i as usize] == b'W' {
                    count += 1;
                }
            }
            result = result.min(count);
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7)
    );
    tracing::info!("{}", Solution::minimum_recolors("WBWBBBW".to_string(), 2));
}
