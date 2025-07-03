impl Solution {
    pub fn possible_string_count(w: String, k: i32) -> i32 {
        let g = w
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|c| c.len() as i64)
            .collect::<Vec<_>>();
        let modulo = 1_000_000_007;
        let mut all = 1;
        for &c in &g {
            all = (all * c) % modulo
        }
        if k as usize <= g.len() {
            return all as i32;
        };
        let n = k as usize - g.len();
        let g = g
            .iter()
            .filter(|&&c| c > 1)
            .map(|&c| c - 1)
            .collect::<Vec<_>>();
        let mut dp = vec![vec![0; n]; 2];
        dp[0][0] = 1i64;
        let mut ps = vec![0; n + 1];
        for i in 0..g.len() {
            for kk in 0..n {
                ps[kk + 1] = (ps[kk] + dp[i % 2][kk]) % modulo;
                dp[1 - (i % 2)][kk] =
                    (ps[kk + 1] - ps[(0.max(kk as i64 - g[i])) as usize]) % modulo;
            }
        }
        let mut bad = 0;
        for i in 0..n {
            bad = (bad + dp[g.len() % 2][i]) % modulo
        }
        ((all + modulo - bad) % modulo) as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::possible_string_count("aabbccdd".to_string(), 7)
    )
}
