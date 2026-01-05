impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let (mut a, mut b, mut c, m) = (1, 1, 2, 1000000007);
        for _ in 3..=n {
            (a, b, c) = (b, c, ((2 * c) % m + a) % m)
        }
        if n < 2 {
            1
        } else if n < 3 {
            2
        } else {
            c
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::num_tilings(3))
}
