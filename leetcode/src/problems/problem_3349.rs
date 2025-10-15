impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        nums.chunk_by(|a, b| a < b)
            .map(|increasing_subarr| increasing_subarr.len() as i32)
            .scan(0, |last_len, cur_len| {
                let ret = (*last_len >= k && cur_len >= k) || cur_len >= 2 * k;
                *last_len = cur_len;
                Some(ret)
            })
            .any(|b| b)
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3)
    )
}
