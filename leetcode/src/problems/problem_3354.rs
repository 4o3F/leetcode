impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut right: i32 = nums.iter().sum();
        let mut left = 0;
        for num in nums {
            result += (num == 0) as i32 * (2 - (right - left).abs()).max(0);
            right -= num;
            left += num;
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_valid_selections(vec![1, 0, 2, 0, 3]))
}
