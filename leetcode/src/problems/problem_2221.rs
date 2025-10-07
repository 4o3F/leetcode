impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() != 1 {
            nums = nums.windows(2).map(|x| (x[0] + x[1]) % 10).collect();
        }
        nums[0]
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::triangular_sum(vec![1, 2, 3, 4, 5]))
}
