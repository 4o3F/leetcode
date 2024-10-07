use std::cmp::Ordering::*;
use std::collections::VecDeque;
struct MyCircularDeque {
    queue: VecDeque<i32>,
}
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            queue: VecDeque::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        match self.queue.len().cmp(&self.queue.capacity()) {
            Equal | Greater => false,
            Less => {
                self.queue.push_front(value);
                true
            }
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        match self.queue.len().cmp(&self.queue.capacity()) {
            Equal | Greater => false,
            Less => {
                self.queue.push_back(value);
                true
            }
        }
    }

    fn delete_front(&mut self) -> bool {
        self.queue.pop_front().is_some()
    }

    fn delete_last(&mut self) -> bool {
        self.queue.pop_back().is_some()
    }

    fn get_front(&self) -> i32 {
        self.queue.front().unwrap_or(&-1).clone()
    }

    fn get_rear(&self) -> i32 {
        self.queue.back().unwrap_or(&-1).clone()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn is_full(&self) -> bool {
        self.queue.len() == self.queue.capacity()
    }
}

pub fn run() {
    let mut obj = MyCircularDeque::new(3);
    tracing::info!("{}", obj.insert_front(1));
    tracing::info!("{}", obj.insert_last(2));
    tracing::info!("{}", obj.insert_front(3));
    tracing::info!("{}", obj.insert_front(4));
    tracing::info!("{}", obj.get_rear());
    tracing::info!("{}", obj.is_full());
    tracing::info!("{}", obj.delete_last());
    tracing::info!("{}", obj.insert_front(4));
    tracing::info!("{}", obj.delete_front());
    tracing::info!("{}", obj.get_front());
    tracing::info!("{}", obj.is_empty());
}
