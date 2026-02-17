impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack = Vec::<usize>::with_capacity(n);

        let mut result = vec![0i32; n];

        for curr_idx in (0..n).rev() {
            while let Some(&top_idx) = stack.last() {
                if temperatures[top_idx] <= temperatures[curr_idx] {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&top_idx) = stack.last() {
                result[curr_idx] = (top_idx - curr_idx) as i32;
            }

            stack.push(curr_idx);
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}
