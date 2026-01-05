use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut pq_min = BinaryHeap::<Reverse<i64>>::new();
        let mut pq_max = BinaryHeap::<i64>::new();
        let mut min_score = *weights.first().unwrap() as i64 + *weights.last().unwrap() as i64;
        let mut max_score = min_score;

        for idx in 0..=(weights.len() as i32) - 2 {
            let idx = idx as usize;
            pq_min.push(Reverse(weights[idx] as i64 + weights[idx + 1] as i64));
            pq_max.push(weights[idx] as i64 + weights[idx + 1] as i64);
        }

        for _ in 0..k - 1 {
            min_score += pq_min.pop().unwrap().0;
            max_score += pq_max.pop().unwrap();
        }
        max_score - min_score
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::put_marbles(vec![1], 1));
    tracing::info!("{}", Solution::put_marbles(vec![1, 3, 5, 1], 2));
    tracing::info!("{}", Solution::put_marbles(vec![1, 3], 2));
}
