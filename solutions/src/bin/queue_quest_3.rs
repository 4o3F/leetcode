#[derive(Debug)]
struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(top) = self.out_stack.pop() {
            top
        } else {
            while let Some(in_top) = self.in_stack.pop() {
                self.out_stack.push(in_top);
            }
            if let Some(top) = self.out_stack.pop() {
                top
            } else {
                unreachable!()
            }
        }
    }

    fn peek(&mut self) -> i32 {
        if let Some(&top) = self.out_stack.last() {
            top
        } else {
            while let Some(in_top) = self.in_stack.pop() {
                self.out_stack.push(in_top);
            }
            if let Some(&top) = self.out_stack.last() {
                top
            } else {
                unreachable!()
            }
        }
    }

    fn empty(&mut self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

use utils::logger::init_logger;

fn main() {
    init_logger();
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    queue.peek();
    queue.pop();
    queue.empty();
    tracing::info!("{:?}", queue);
}
