impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let length = nums.len();

        let mut count = 0;
        for i in 0..length {
            for j in i + 1..length {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                    count += 1;
                    // tracing::info!("{} {}", i, j);
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
}
