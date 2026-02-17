use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let stone1 = heap.pop().unwrap();
            let stone2 = heap.pop().unwrap();
            if stone1 != stone2 {
                heap.push(stone1.abs_diff(stone2) as i32);
            }
        }
        heap.pop().unwrap_or(0)
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]))
}
