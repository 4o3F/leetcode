impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_unstable_by_key(|x| x[0]);

        let mut result = Vec::<Vec<i32>>::with_capacity(intervals.len());

        for interval in intervals.into_iter() {
            match result.last_mut() {
                Some(last) if interval[0] <= last[1] => {
                    last[1] = last[1].max(interval[1]);
                }
                _ => result.push(interval),
            }
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
        Solution::merge([[1, 4], [2, 3]].iter().map(Vec::from).collect())
    )
}
