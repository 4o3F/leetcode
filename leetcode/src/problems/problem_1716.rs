impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut result = 0;
        for i in 0..(n / 7) {
            result += ((1 + i) * 2 + 6) * 7 / 2;
        }
        result += (((n / 7) + 1) * 2 + (n % 7) - 1) * (n % 7) / 2;
        result
    }
}

struct Solution;

pub fn run() {
    // tracing::info!("{}", Solution::total_money(4));
    tracing::info!("{}", Solution::total_money(10));
    tracing::info!("{}", Solution::total_money(20));
}
