impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let base = i64::from((n - 1) * 2 + 1);
        let n = i64::from(n);
        2 * ((base + 1) * n / 2) - base
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::colored_cells(2));
}
