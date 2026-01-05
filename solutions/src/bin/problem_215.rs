use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums);
        for _ in 0..k - 1 {
            heap.pop();
        }
        heap.pop().unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2))
}
