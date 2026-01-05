impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut freq = [0; 26];
        for b in word.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let k = k as usize;
        let max_freq = *freq.iter().max().unwrap();
        let mut best = word.len() as i32;

        for f_min in 0..=max_freq {
            let f_max = f_min + k;
            let mut deletions = 0;
            for &f in &freq {
                if f < f_min {
                    deletions += f;
                } else if f > f_max {
                    deletions += f - f_max;
                }
            }
            best = best.min(deletions as i32);
        }

        best
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::minimum_deletions("aabcaba".to_string(), 0))
}
