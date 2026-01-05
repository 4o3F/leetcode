use std::collections::VecDeque;

impl Solution {
    pub fn check_valid_cuts(_n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        // Sort by start x
        rectangles.sort_unstable_by_key(|x| x[0]);

        let mut queue = VecDeque::from(rectangles.clone());
        let item = queue.pop_front().unwrap();
        let mut end = item[2];
        let mut count = 0;
        while let Some(item) = queue.pop_front() {
            if item[0] < end {
                end = end.max(item[2]);
            } else {
                count += 1;
                end = item[2];
            }
        }

        if count >= 2 {
            return true;
        }

        // Sort by start y
        rectangles.sort_unstable_by_key(|x| x[1]);

        let mut queue = VecDeque::from(rectangles.clone());
        let item = queue.pop_front().unwrap();
        let mut end = item[3];
        let mut count = 0;
        while let Some(item) = queue.pop_front() {
            if item[1] < end {
                end = end.max(item[3]);
            } else {
                count += 1;
                end = item[3];
            }
        }

        if count >= 2 {
            return true;
        }
        false
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5]
            ]
        )
    )
}
