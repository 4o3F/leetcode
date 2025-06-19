impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut start_idx = 0;
        let mut result = 0;
        for (end_idx, current_num) in nums.iter().enumerate() {
            if current_num - nums[start_idx] > k {
                start_idx = end_idx;
                result += 1;
            }
        }
        result + 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::partition_array(vec![3, 6, 1, 2, 5], 2))
}
