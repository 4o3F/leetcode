use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        // (rot_day, remain_count)
        let mut heap = BinaryHeap::<Reverse<(usize, i32)>>::new();

        let mut day = 0;

        let mut result = 0;

        while day < apples.len() || !heap.is_empty() {
            if day < apples.len() && apples[day] > 0 {
                heap.push(Reverse((day + days[day] as usize, apples[day])));
            }

            while let Some(Reverse((rot_day, _))) = heap.peek() {
                if *rot_day <= day {
                    heap.pop();
                } else {
                    break;
                }
            }

            if let Some(Reverse((rot_day, remain_count))) = heap.pop() {
                result += 1;
                if remain_count > 1 {
                    heap.push(Reverse((rot_day, remain_count - 1)));
                }
            }

            day += 1;
        }

        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::eaten_apples(vec![2, 1, 1, 4, 5], vec![10, 10, 6, 4, 2])
    );
    // tracing::info!(
    //     "{}",
    //     Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2])
    // );
}
