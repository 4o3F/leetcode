impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        for gain in gain {
            sum += gain;
            result = result.max(sum);
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
}
