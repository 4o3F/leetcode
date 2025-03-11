use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[derive(Debug)]
struct Worker {
    is_start: bool,
    cost: i64,
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.is_start == other.is_start && self.cost == other.cost
    }
}

impl Eq for Worker {}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut queue = VecDeque::from_iter(costs.into_iter().map(i64::from));
        let mut heap = BinaryHeap::<Reverse<Worker>>::new();
        for _ in 0..candidates {
            if queue.len() == 0 {
                break;
            }
            heap.push(Reverse(Worker {
                is_start: true,
                cost: queue.pop_front().unwrap(),
            }));
            if queue.len() == 0 {
                break;
            }
            heap.push(Reverse(Worker {
                is_start: false,
                cost: queue.pop_back().unwrap(),
            }));
        }
        // tracing::info!("{:?}", queue);
        let mut total_cost = 0i64;
        for _ in 0..k {
            // tracing::info!("{:?}", heap);
            // tracing::info!("{:?}", queue);
            let worker = heap.pop().unwrap().0;
            total_cost += worker.cost;

            if worker.is_start {
                if queue.len() > 0 {
                    heap.push(Reverse(Worker {
                        is_start: true,
                        cost: queue.pop_front().unwrap(),
                    }));
                }
            } else {
                if queue.len() > 0 {
                    heap.push(Reverse(Worker {
                        is_start: false,
                        cost: queue.pop_back().unwrap(),
                    }));
                }
            }
        }
        total_cost
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
    );
    tracing::info!("{}", Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
}
