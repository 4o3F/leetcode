use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        nums.iter()
            .fold(HashMap::<i32, i32>::new(), |mut f, n| {
                *f.entry(*n).or_default() += 1;
                f
            })
            .into_values()
            .all(|x| x % 2 == 0)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
}
