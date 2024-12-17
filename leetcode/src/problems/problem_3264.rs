use std::collections::BinaryHeap;

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut h = BinaryHeap::from_iter(nums.iter().enumerate().map(|(i, &x)| (-x, -(i as i32))));
        for _ in 0..k {
            let (x, i) = h.pop().unwrap();
            nums[(-i) as usize] *= multiplier;
            h.push((x * multiplier, i));
        }
        nums
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{:?}", Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
}
