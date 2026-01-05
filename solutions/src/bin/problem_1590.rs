use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let (mut sum, mut wait) = (0, HashMap::new());
        wait.insert(0, -1);
        let target = (nums.iter().map(|&x| (x % p) as i64).sum::<i64>() % (p as i64)) as i32;
        let ans = nums
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                sum = (sum + n % p) % p;
                wait.insert(sum, i as i32);
                let key = (p + sum - target) % p;
                if let Some(j) = wait.get(&key) {
                    i as i32 - j
                } else {
                    nums.len() as i32
                }
            })
            .min()
            .unwrap();
        if ans < nums.len() as i32 {
            ans
        } else {
            -1
        }
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let nums = vec![3, 1, 4, 1, 5];
    let p = 5;
    tracing::info!("{:?}", Solution::min_subarray(nums, p));
}
