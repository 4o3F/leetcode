impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n_usz = n as usize;
        let mut powers: Vec<usize> = Vec::new();

        let mut i: usize = 1;
        loop {
            let mut p: usize = 1;
            for _ in 0..x {
                p = p.saturating_mul(i);
            }
            if p > n_usz {
                break;
            }
            powers.push(p);
            i += 1;
        }

        let mut dp = vec![0u64; n_usz + 1];
        dp[0] = 1;
        for &p in &powers {
            for s in (p..=n_usz).rev() {
                dp[s] = (dp[s] + dp[s - p]) % MOD;
            }
        }

        dp[n_usz] as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::number_of_ways(10, 2))
}
