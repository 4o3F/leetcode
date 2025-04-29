impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_count = 0;
        let max_num = nums.iter().max().unwrap();
        let mut left_idx = 0;
        let mut result = 0;
        for (_, num) in nums.iter().enumerate() {
            if num == max_num {
                max_count += 1;
            }
            while max_count >= k {
                if &nums[left_idx] == num {
                    max_count -= 1;
                }
                left_idx += 1;
            }
            result += left_idx as i64;
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2));
}
