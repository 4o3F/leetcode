impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        const MASK: i32 = 0x5555_5555;
        n & -n == n && n & MASK != 0
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::is_power_of_four(16))
}
