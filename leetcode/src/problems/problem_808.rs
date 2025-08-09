impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 5000 {
            return 1.0;
        }
        let n = ((n + 124) / 25) as usize;
        let mut p = vec![vec![0.0; n]; n];
        for j in 0..5 {
            for i in j..n {
                p[j][i] = 1.0;
                p[j][j] = 0.5
            }
        }
        for a in 4..n {
            for b in 4..n {
                p[a][b] = 0.25 * (p[a - 4][b] + p[a - 3][b - 1] + p[a - 2][b - 2] + p[a - 1][b - 3])
            }
        }
        p[n - 1][n - 1]
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::soup_servings(50))
}
