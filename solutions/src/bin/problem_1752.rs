impl Solution {
    pub fn check(n: Vec<i32>) -> bool {
        (0..n.len())
            .filter(|&i| n[i] > n[(i + 1) % n.len()])
            .count()
            < 2
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::check(vec![2, 1, 3, 4]));
}
