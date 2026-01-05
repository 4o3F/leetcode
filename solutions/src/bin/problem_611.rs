impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut count = 0;

        for k in (2..=nums.len() - 1).rev() {
            let (mut i, mut j) = (0, k - 1);
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    count += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        count as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::triangle_number(vec![2, 2, 3, 4]))
}
