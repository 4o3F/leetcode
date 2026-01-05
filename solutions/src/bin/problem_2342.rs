use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(
                HashMap::<i32, BinaryHeap<i32>>::new(),
                |mut digit_sum_map, num| {
                    let mut digit_sum = 0;
                    let mut tmp = *num;
                    while tmp > 0 {
                        digit_sum += tmp % 10;
                        tmp /= 10;
                    }
                    digit_sum_map.entry(digit_sum).or_default().push(*num);
                    digit_sum_map
                },
            )
            .into_values()
            .map(|mut v| {
                if v.len() < 2 {
                    -1
                } else {
                    v.pop().unwrap() + v.pop().unwrap()
                }
            })
            .max()
            .unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::maximum_sum(vec![
            229, 398, 269, 317, 420, 464, 491, 218, 439, 153, 482, 169, 411, 93, 147, 50, 347, 210,
            251, 366, 401
        ])
    );
    tracing::info!("{}", Solution::maximum_sum(vec![10, 12, 19, 14]));
    tracing::info!("{}", Solution::maximum_sum(vec![18, 43, 36, 13, 7]));
}
