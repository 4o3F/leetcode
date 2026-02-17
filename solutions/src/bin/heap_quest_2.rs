use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // sum, num1, num2
        let mut heap = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();
        for i in 0..nums1.len().min(k as usize) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }
        let mut result = Vec::<Vec<i32>>::with_capacity(k as usize);

        while result.len() < k as usize {
            // dbg!(&heap);
            let Some(Reverse(top)) = heap.pop() else {
                unreachable!()
            };
            result.push(vec![nums1[top.1], nums2[top.2]]);
            if top.2 + 1 < nums2.len() {
                heap.push(Reverse((nums1[top.1] + nums2[top.2 + 1], top.1, top.2 + 1)));
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
        Solution::k_smallest_pairs(vec![1, 2], vec![1, 2, 9], 3)
    );
}
