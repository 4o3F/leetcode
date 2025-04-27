use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut vals = HashMap::new();
        let (mut res, mut cnt) = (0i64, 0);
        vals.insert(0, 1);

        for v in nums {
            if v % modulo == k {
                cnt += 1;
            }
            let pos = (cnt - k + modulo) % modulo;
            match vals.get(&pos) {
                Some(i) => res += *i as i64,
                _ => (),
            }

            *vals.entry(cnt % modulo).or_insert(0) += 1;
        }

        res
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1)
    );
}
