use std::collections::BinaryHeap;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 0;
        let mut events: Vec<(i32, i32)> = events.into_iter().map(|e| (e[1], e[0])).collect();
        events.sort();
        let mut cur_events: BinaryHeap<i32> = BinaryHeap::with_capacity(events.len());
        let mut d: i32 = i32::MAX;
        while let Some(e) = events.pop() {
            while d > e.0 {
                if let Some(x) = cur_events.pop() {
                    if d >= x {
                        count += 1;
                        d -= 1;
                    }
                } else {
                    break;
                }
            }
            if e.0 < d {
                d = e.0;
            }
            cur_events.push(e.1);
        }
        while let Some(x) = cur_events.pop() {
            if d >= x {
                count += 1;
                d -= 1;
            }
        }
        count
    }
}
struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]])
    )
}
