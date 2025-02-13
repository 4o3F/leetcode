use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut pq = BinaryHeap::from_iter(nums.into_iter().map(i64::from).map(Reverse));
        let k = k as i64;
        while !pq.is_empty() && pq.len() >= 2 && pq.peek().unwrap().0 < k {
            // tracing::info!("{:?}", pq);
            let (a, b) = (pq.pop().unwrap().0, pq.pop().unwrap().0);
            pq.push(Reverse(a.min(b) * 2 + a.max(b)));
            count += 1;
        }
        count
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::min_operations(
            vec![1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999],
            1000000000
        )
    );
    tracing::info!("{}", Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
}
