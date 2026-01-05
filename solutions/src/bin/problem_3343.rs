impl Solution {
    pub fn count_balanced_permutations(s: String) -> i32 {
        let (mut sum, mut f, mut c, m) = (0, [0; 10], [[0; 81]; 81], 1000000007);
        for b in s.bytes() {
            f[(b - b'0') as usize] += 1;
            sum += (b - b'0') as i64
        }
        if sum & 1 > 0 {
            return 0;
        };
        let mut dp = [[[-1; 81]; 81]; 361];
        for i in 0..81 {
            c[i][0] = 1;
            for j in 1..=i {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % m
            }
        }
        fn dfs(
            i: i64,
            o: i64,
            e: i64,
            b: i64,
            c: &[[i64; 81]; 81],
            dp: &mut [[[i64; 81]; 81]; 361],
            f: &[i64; 10],
        ) -> i64 {
            if o == 0 && e == 0 && b == 0 {
                return 1;
            } else if i.min(o).min(e).min(b) < 0 {
                return 0;
            }
            if dp[b as usize][i as usize][o as usize] >= 0 {
                return dp[b as usize][i as usize][o as usize];
            };
            let (mut r, m) = (0, 1000000007);
            for j in 0..=f[i as usize] {
                let k = f[i as usize] - j;
                let comb = (c[o as usize][j as usize] * c[e as usize][k as usize]) % m;
                let next = dfs(i - 1, o - j, e - k, b - i * j, c, dp, f);
                r = (r + (comb * next) % m) % m
            }
            dp[b as usize][i as usize][o as usize] = r;
            r
        }
        dfs(
            9,
            (1 + s.len() as i64) / 2,
            s.len() as i64 / 2,
            sum / 2,
            &c,
            &mut dp,
            &f,
        ) as _
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_balanced_permutations("123".to_string())
    )
}
