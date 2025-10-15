const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        // Pascal's triangle
        let mut c = vec![vec![0i64; 31]; 31];
        for i in 1..=30 {
            c[i][0] = 1;
            c[i][i] = 1;
            for j in 1..=i / 2 {
                let val = (c[i - 1][j - 1] + c[i - 1][j]) % MOD;
                c[i][j] = val;
                c[i][i - j] = val;
            }
        }

        use std::collections::HashMap;

        fn dfs(
            m: i32,
            k: i32,
            i: usize,
            flag: u32,
            nums: &Vec<i32>,
            c: &Vec<Vec<i64>>,
            memo: &mut HashMap<(i32, i32, usize, u32), i64>,
        ) -> i64 {
            if let Some(&res) = memo.get(&(m, k, i, flag)) {
                return res;
            }

            let bz = flag.count_ones() as i32;
            if m < 0 || k < 0 || m + bz < k {
                return 0;
            }
            if m == 0 {
                return if k == bz { 1 } else { 0 };
            }
            if i >= nums.len() {
                return 0;
            }

            let mut ans = 0i64;
            let mut pow_x = 1i64;
            let x = nums[i] as i64;

            for f in 0..=m {
                let perm = (c[m as usize][f as usize] * pow_x) % MOD;
                let new_flag = flag + f as u32;
                let next_flag = new_flag >> 1;
                let bit_set = (new_flag & 1) as i32;
                ans = (ans + perm * dfs(m - f, k - bit_set, i + 1, next_flag, nums, c, memo)) % MOD;
                pow_x = (pow_x * x) % MOD;
            }

            memo.insert((m, k, i, flag), ans);
            ans
        }

        let mut memo = HashMap::new();
        dfs(m, k, 0, 0, &nums, &c, &mut memo) as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000])
    )
}
