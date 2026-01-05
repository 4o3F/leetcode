use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut string = String::with_capacity((a + b + c) as usize);
        let mut heap = BinaryHeap::with_capacity(3);

        if a > 0 {
            heap.push((a, 'a'));
        }

        if b > 0 {
            heap.push((b, 'b'));
        }

        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut total = a + b + c;
        let mut last = None;
        while let Some((mut count, ch)) = heap.pop() {
            string.push(ch);

            count -= 1;
            total -= 1;
            if count > (total - count) * 2 {
                count -= 1;
                total -= 1;
                string.push(ch);
            }
            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }
        string
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::longest_diverse_string(1, 1, 7));
}
