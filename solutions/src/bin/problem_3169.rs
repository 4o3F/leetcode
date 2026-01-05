use std::collections::VecDeque;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|m| m[0]);
        let mut meetings = VecDeque::from(meetings);

        let meeting = meetings.pop_front().unwrap();
        let mut end = meeting[1];

        let mut result = meeting[0] - 1;
        while let Some(meeting) = meetings.pop_front() {
            if meeting[0] <= end {
                end = end.max(meeting[1]);
            } else {
                result += meeting[0] - end - 1;
                // tracing::info!("last end {} result {}", end, result);
                end = meeting[1];
            }
        }

        result += days - end;
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]])
    );
    tracing::info!("{}", Solution::count_days(5, vec![vec![2, 4], vec![1, 3]]));
}
