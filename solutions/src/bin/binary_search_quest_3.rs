impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mid = c.isqrt();
        for first_num in 0..=mid {
            let target = c - first_num * first_num;
            let target_sqrt = target.isqrt();
            if target_sqrt * target_sqrt == target {
                return true;
            }
        }
        false
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::judge_square_sum(2));
    tracing::info!("{}", Solution::judge_square_sum(3));
}
