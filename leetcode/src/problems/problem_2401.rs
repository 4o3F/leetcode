impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut mask = 0;
        let mut result = 0;

        while right < nums.len() {
            if mask & nums[right] == 0 {
                result = result.max(right - left + 1);
            } else {
                while left < right && mask & nums[right] != 0 {
                    mask ^= nums[left];
                    left += 1;
                }
            }
            mask ^= nums[right];

            right += 1;
        }

        result as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
}
