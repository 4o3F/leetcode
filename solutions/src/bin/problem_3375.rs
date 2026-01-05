use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut exist = HashSet::<i32>::new();
        let mut count = 0;
        for num in nums {
            if num < k {
                return -1;
            } else if num > k && !exist.contains(&num) {
                exist.insert(num);
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_operations(vec![5, 2, 5, 4, 5], 2));
}
