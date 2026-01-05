use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    num2idx: HashMap<i32, BTreeSet<i32>>,
    idx2num: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            num2idx: HashMap::new(),
            idx2num: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(num) = self.idx2num.get_mut(&index) {
            self.num2idx.entry(*num).or_default().remove(&index);
        }
        self.num2idx.entry(number).or_default().insert(index);
        self.idx2num.insert(index, number);
    }

    fn find(&self, number: i32) -> i32 {
        // tracing::info!("{:?}", self.num2idx);
        if let Some(idx) = self.num2idx.get(&number) {
            *idx.iter().next().unwrap_or(&-1)
        } else {
            -1
        }
    }
}

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut obj = NumberContainers::new();
    tracing::info!("{}", obj.find(10));
    obj.change(2, 10);
    obj.change(1, 10);
    obj.change(3, 10);
    obj.change(5, 10);
    tracing::info!("{}", obj.find(10));
    obj.change(1, 20);
    tracing::info!("{}", obj.find(10));
}
