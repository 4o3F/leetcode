use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::<i32>::new();
        for num in nums {
            if num <= 0 {
                continue;
            }
            set.insert(num);
        }
        let mut smallest = 1;

        while set.contains(&smallest) {
            smallest += 1;
        }

        smallest
    }
}

struct Solution;
use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::first_missing_positive(vec![7, 8, 9, 11, 12])
    )
}
