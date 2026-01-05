impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        (1 << (32 - n.leading_zeros())) - 1
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::smallest_number(5))
}
