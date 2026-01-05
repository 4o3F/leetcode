impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts: Vec<i32> = Vec::with_capacity(intervals.len());
        let mut ends: Vec<i32> = Vec::with_capacity(intervals.len());

        for interval in &intervals {
            starts.push(interval[0]);
            ends.push(interval[1]);
        }

        starts.sort_unstable();
        ends.sort_unstable();

        let mut total = 0;
        let mut max_total = 0;
        let mut start_idx = 0;
        let mut end_idx = 0;

        while start_idx < intervals.len() {
            if starts[start_idx] <= ends[end_idx] {
                total += 1;
                start_idx += 1;
                max_total = max_total.max(total);
            } else {
                total -= 1;
                end_idx += 1;
            }
        }

        max_total
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    let intervals = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
    tracing::info!("{:?}", Solution::min_groups(intervals));
}
