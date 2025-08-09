impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::is_power_of_two(16))
}
