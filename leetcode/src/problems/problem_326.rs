impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        1162261467 % n == 0
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::is_power_of_three(45))
}
