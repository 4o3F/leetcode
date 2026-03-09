impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        for window in arr.windows(2) {
            min_diff = min_diff.min(window[1] - window[0]);
        }
        arr.windows(2)
            .filter(|&window| window[1] - window[0] == min_diff)
            .map(Vec::from)
            .collect::<Vec<_>>()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::minimum_abs_difference(vec![4, 2, 1, 3]));
}
