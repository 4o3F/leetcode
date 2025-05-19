impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        const M: i32 = 1000000007;
        let mut ms = vec![];
        fn msk(i: i32, c: i32, p: i32, ms: &mut Vec<i32>) {
            if i < 1 {
                ms.push(c);
                return;
            }
            for j in 1..4 {
                if j != p {
                    msk(i - 1, (c << 3) | (1 << j), j, ms)
                }
            }
        }
        msk(m, 0, 0, &mut ms);
        let s = ms.len();
        let (mut dp, mut dp2) = (vec![1; s], vec![0; s]);
        for _ in 1..n {
            for mask in 0..s {
                let mut c = 0;
                for m in 0..s {
                    if (ms[m] & ms[mask]) == 0 {
                        c = (c + dp[m]) % M
                    }
                }
                dp2[mask] = c
            }
            (dp, dp2) = (dp2, dp)
        }
        dp.into_iter().fold(0, |r, t| (r + t) % M)
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::color_the_grid(5, 5))
}
