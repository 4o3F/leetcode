use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, &x) in nums.iter().enumerate() {
            let need = target - x;
            if let Some(&j) = map.get(&need) {
                return vec![j, i as i32];
            }
            map.insert(x, i as i32);
        }
        unreachable!()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}
