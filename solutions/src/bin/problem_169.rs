use utils::logger::init_logger;

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, usize>::new();
        let threshold = nums.len() / 2;
        for num in nums {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
            if *entry > threshold {
                return num;
            }
        }
        unreachable!()
    }
}

struct Solution;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
