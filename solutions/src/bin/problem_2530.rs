impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(nums);

        (0..k).fold(0, |num, _| {
            let value = heap.pop().unwrap();
            heap.push((value + 2) / 3);
            num + value as i64
        })
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_kelements(vec![1, 10, 3, 3, 3], 3));
}
