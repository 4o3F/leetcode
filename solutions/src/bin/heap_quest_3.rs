use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let target = target.iter().map(|&x| i64::from(x)).collect::<Vec<i64>>();
        let mut total_sum = target.iter().sum::<i64>();

        let mut heap = BinaryHeap::from(target);

        while let Some(largest) = heap.pop()
            && largest != 1
        {
            dbg!(&heap);
            let rest = total_sum - largest;
            if rest == 0 || largest <= rest {
                return false;
            } else if rest == 1 {
                return true;
            }
            let previous = largest % rest;
            if previous == 0 {
                return false;
            }
            total_sum = rest + previous;
            heap.push(previous);
        }

        true
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::is_possible(vec![2, 900000002]));
    // tracing::info!("{}", Solution::is_possible(vec![8, 5]));
    // tracing::info!("{}", Solution::is_possible(vec![1, 1, 1, 2]));
    // tracing::info!("{}", Solution::is_possible(vec![9, 3, 5]));
}
