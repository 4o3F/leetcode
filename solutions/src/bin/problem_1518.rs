impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::num_water_bottles(9, 3));
}
