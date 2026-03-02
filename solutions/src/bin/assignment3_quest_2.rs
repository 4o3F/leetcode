struct MyCircularQueue {
    buf: Vec<i32>,
    head: usize,
    tail: usize,
    count: usize,
    cap: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let cap = k as usize;
        Self {
            buf: vec![0; cap],
            head: 0,
            tail: 0,
            count: 0,
            cap,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.tail] = value;
        self.tail = (self.tail + 1) % self.cap;
        self.count += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.cap;
        self.count -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.buf[self.head]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let idx = (self.tail + self.cap - 1) % self.cap;
        self.buf[idx]
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.cap
    }
}

use utils::logger::init_logger;

fn main() {
    init_logger();
    let mut queue = MyCircularQueue::new(3);
    queue.en_queue(1);
    queue.en_queue(2);
    queue.en_queue(3);
    queue.en_queue(4);
    queue.rear();
    queue.is_full();
    queue.de_queue();
    queue.en_queue(4);
    queue.rear();
    queue.front();
}
