impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let (mut left, mut right) = (0, *candies.iter().max().unwrap());
        while left < right {
            let mid = (left + right + 1) / 2;
            if Self::can_allocate_candies(&candies, k, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }

    fn can_allocate_candies(candies: &[i32], k: i64, num_candies: i32) -> bool {
        let mut total_child: i64 = 0;
        for &p in candies.iter() {
            total_child += (p / num_candies) as i64;
            if total_child >= k {
                return true;
            }
        }
        total_child >= k
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_candies(vec![5, 8, 6], 3));
}
