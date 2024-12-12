use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        (0..k)
            .fold(BinaryHeap::from_iter(gifts.into_iter()), |mut heap, _| {
                let value = heap.pop().unwrap();
                heap.push(f64::from(value).sqrt().floor() as i32);
                heap
            })
            .into_iter()
            .map(|x| x as i64)
            .sum()
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4));
    tracing::info!("{}", Solution::pick_gifts(vec![1, 1, 1, 1], 4));
}
