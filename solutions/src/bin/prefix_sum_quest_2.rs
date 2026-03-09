use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let need = sum % p;

        if need == 0 {
            return 0;
        }

        let mut last_idx = HashMap::<i64, i32>::new();
        last_idx.insert(0, 0);

        let mut ans = nums.len() as i32 + 1;
        let mut prefix = 0i64;

        for (idx, &num) in nums.iter().enumerate() {
            let idx = (idx + 1) as i32;
            prefix = (prefix + num as i64) % p;
            let target = (prefix - need + p) % p;
            if let Some(&j) = last_idx.get(&target) {
                ans = ans.min(idx - j);
            }
            last_idx.insert(prefix, idx);
        }

        if ans > nums.len() as i32 || ans == nums.len() as i32 {
            -1
        } else {
            ans
        }
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::min_subarray(vec![1, 2, 3], 7));
}
