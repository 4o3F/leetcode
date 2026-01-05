use std::{collections::HashMap, iter::repeat_n};

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut min = i32::MAX;
        basket1
            .into_iter()
            .zip(basket2)
            .fold(HashMap::<i32, i32>::new(), |mut map, (b1, b2)| {
                min = min.min(b1).min(b2);
                *map.entry(b1).or_default() += 1;
                *map.entry(b2).or_default() -= 1;
                map
            })
            .into_iter()
            .try_fold(Vec::new(), |mut extra, (k, v)| {
                (v % 2 == 0).then(|| {
                    extra.extend(repeat_n(k, v.unsigned_abs() as usize / 2));
                    extra
                })
            })
            .map_or(-1, |mut extra| {
                let len = extra.len() / 2;
                extra.sort_unstable();
                min *= 2;
                extra.into_iter().take(len).map(|v| v.min(min) as i64).sum()
            })
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::min_cost(
            vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88],
            vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8]
        )
    );
}
