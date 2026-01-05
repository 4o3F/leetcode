use std::collections::{BinaryHeap, HashSet};

struct SmallestInfiniteSet {
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
    smallest: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            heap: BinaryHeap::new(),
            set: HashSet::new(),
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            Some(n) => {
                self.set.remove(&n);
                -n
            }
            None => {
                self.smallest += 1;
                self.smallest - 1
            }
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.smallest {
            return;
        }
        if !self.set.contains(&(-num)) {
            self.heap.push(-num);
            self.set.insert(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
fn main() {
    use utils::prelude::*;
    init_logger();
    let mut set = SmallestInfiniteSet::new();
    set.add_back(2);
    tracing::info!("{}", set.pop_smallest());
    tracing::info!("{}", set.pop_smallest());
    tracing::info!("{}", set.pop_smallest());
    set.add_back(1);
    tracing::info!("{}", set.pop_smallest());
    tracing::info!("{}", set.pop_smallest());
    tracing::info!("{}", set.pop_smallest());
}
