impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();

        let count_pairs = |max| {
            let mut pairs = 0;
            let mut i = 1;
            while i < nums.len() {
                if (nums[i] - nums[i - 1]) <= max {
                    pairs += 1;
                    i += 1;
                }
                i += 1;
            }
            pairs
        };

        let mut lo = 0;
        let mut hi = *nums.last().unwrap() - *nums.first().unwrap();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if count_pairs(mid) >= p {
                hi = mid
            } else {
                lo = mid + 1
            }
        }

        lo
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2));
}
