impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(n as usize);
        result.push(1);
        while result.len() < n as usize {
            result = result
                .iter()
                .map(|m| 2 * m - 1)
                .chain(result.iter().map(|m| 2 * m))
                .filter(|&m| m <= n)
                .collect();
        }

        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::beautiful_array(4))
}
