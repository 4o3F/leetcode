impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut odd_count = 0;
        let mut even_count = 0;
        let mut result = 0;
        for num in arr {
            (odd_count, even_count) = match num % 2 {
                0 => (odd_count, even_count + 1),
                _ => (even_count + 1, odd_count),
            };
            result = (result + odd_count) % MOD;
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::num_of_subarrays(vec![1, 3, 5]));
}
