impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (mut f, mut res, mut sum, mut j) = ([0; 100_001], 0, 0, 0);
        for (i, &n) in nums.iter().enumerate() {
            while i - j + 1 > k as usize || f[n as usize] > 0 {
                sum -= nums[j] as i64;
                f[nums[j] as usize] -= 1;
                j += 1
            }
            sum += n as i64;
            f[n as usize] += 1;
            if i - j + 1 == k as usize {
                res = res.max(sum)
            }
        }
        res
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let arr = vec![5, 3, 3, 1, 1];
    tracing::info!("{}", Solution::maximum_subarray_sum(arr, 3));
    let arr = vec![1, 2, 2];
    tracing::info!("{}", Solution::maximum_subarray_sum(arr, 2));
    // 15
    let arr = vec![1, 5, 4, 2, 9, 9, 9];
    tracing::info!("{}", Solution::maximum_subarray_sum(arr, 3));
    // 24
    let arr = vec![1, 1, 1, 7, 8, 9];
    tracing::info!("{}", Solution::maximum_subarray_sum(arr, 3));
    // 12
    let arr = vec![9, 9, 9, 1, 2, 3];
    tracing::info!("{}", Solution::maximum_subarray_sum(arr, 3));
}
