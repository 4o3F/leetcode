use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_ors(arr: Vec<i32>) -> i32 {
        let mut res = HashSet::new();
        let mut cur = HashSet::new();

        for &num in &arr {
            let mut next = HashSet::new();
            next.insert(num);
            for &x in &cur {
                next.insert(x | num);
            }
            cur = next;
            for x in &cur {
                res.insert(*x);
            }
        }
        res.len() as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::subarray_bitwise_ors(vec![1, 1, 2]))
}
