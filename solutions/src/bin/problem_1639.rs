impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let mut dp = vec![0; target.len() + 1];
        dp[target.len()] = 1;
        let m = 1_000_000_007i64;
        let target: Vec<_> = target.bytes().rev().collect();
        for pos_f in 0..words[0].len() {
            let mut fr = vec![0; 26];
            for w in &words {
                fr[(w.as_bytes()[pos_f] - b'a') as usize] += 1
            }
            for (pos_t, t) in target.iter().enumerate() {
                dp[pos_t] += fr[(t - b'a') as usize] * dp[pos_t + 1] % m
            }
        }
        (dp[0] % m) as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::num_ways(
            vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
            "aba".to_string()
        )
    )
}
