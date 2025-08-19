impl Solution {
    pub fn maximum69_number(n: i32) -> i32 {
        n + 3 * [1000, 100, 10, 1]
            .iter()
            .find(|&&p| n / p % 10 == 6)
            .unwrap_or(&0)
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum69_number(9669))
}
