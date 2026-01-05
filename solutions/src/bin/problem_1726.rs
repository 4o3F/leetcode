use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::<i32, u32>::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                *freq.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }
        // tracing::info!("{:?}", freq);
        freq.iter()
            .filter(|(_, &f)| f > 1)
            .map(|(_, count)| (count * (count - 1) / 2) * 8)
            .sum::<u32>() as i32
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::tuple_same_product(vec![2, 3, 4, 6]));
    tracing::info!("{}", Solution::tuple_same_product(vec![1, 2, 4, 5, 10]));
}
