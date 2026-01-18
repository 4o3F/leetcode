use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        *nums
            .iter()
            .fold(HashMap::<i32, u8>::new(), |mut map, num| {
                let entry = map.entry(*num).or_insert(0);
                *entry += 1;
                map
            })
            .iter()
            .find(|&(_, &v)| v != 2)
            .unwrap()
            .0
    }
}

use utils::logger::init_logger;

struct Solution;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::single_number(vec![4, 1, 2, 1, 2]));
    tracing::info!("{}", Solution::single_number(vec![2, 2, 1]));
    tracing::info!("{}", Solution::single_number(vec![1]));
}
