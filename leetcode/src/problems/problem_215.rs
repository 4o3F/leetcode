use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums.into_iter().map(|x| x));
        for _ in 0..k - 1 {
            heap.pop();
        }
        heap.pop().unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2))
}
