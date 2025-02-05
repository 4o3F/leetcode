impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            // tracing::info!("{} {} {} {}", nums[i], nums[i+1], nums[i] / 2, nums[i+1] / 2);
            if nums[i] % 2 == nums[i + 1] % 2 {
                return false;
            }
        }
        return true;
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::is_array_special(vec![4, 3, 1, 6]))
}
