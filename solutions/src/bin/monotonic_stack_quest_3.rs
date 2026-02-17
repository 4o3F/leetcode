impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights.clone();
        heights.push(0);
        let n = heights.len();
        let mut stack = Vec::<usize>::with_capacity(n);
        let mut result = 0;
        for curr_idx in 0..n {
            while let Some(&top_idx) = stack.last() {
                if heights[curr_idx] < heights[top_idx] {
                    stack.pop();
                    let height = heights[top_idx];
                    let left = stack.last().copied();
                    let width = match left {
                        Some(left) => curr_idx - left - 1,
                        None => curr_idx,
                    } as i32;
                    result = result.max(width * height);
                } else {
                    break;
                }
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
        "{}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
    tracing::info!("{}", Solution::largest_rectangle_area(vec![2, 4]));
}
