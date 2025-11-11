impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in strs {
            let (zero, one) = s.bytes().fold((0, 0), |(zero, one), ch| match ch {
                b'0' => (zero + 1, one),
                _ => (zero, one + 1),
            });

            for next_m in (zero..=m).rev() {
                for next_n in (one..=n).rev() {
                    dp[next_m][next_n] =
                        dp[next_m][next_n].max(dp[next_m - zero][next_n - one] + 1);
                }
            }
        }

        dp.into_iter().flatten().max().unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        )
    );
}
