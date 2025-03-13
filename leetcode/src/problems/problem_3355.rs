impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let len = nums.len();
        let diff_array = queries
            .iter()
            .fold(vec![0; len + 1], |mut diff_array, query| {
                diff_array[query[0] as usize] -= 1;
                diff_array[query[1] as usize + 1] += 1;
                diff_array
            });

        let mut decrement = 0;
        for (idx, num) in nums.iter().enumerate() {
            decrement += diff_array[idx];
            if num + decrement > 0 {
                return false;
            }
        }
        true
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]])
    );
}
