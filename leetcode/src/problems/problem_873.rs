use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut dp = vec![vec![0; arr.len()]; arr.len()];
        let mut idx_memo = HashMap::<i32, usize>::new();
        for i in 0..arr.len() {
            idx_memo.insert(arr[i], i);
            for j in 0..i {
                // Check if the first num exists
                let k = match idx_memo.get(&(arr[i] - arr[j])) {
                    Some(idx) => *idx as i32,
                    None => -1,
                };
                dp[j][i] = if k >= 0 && arr[i] - arr[j] < arr[j] {
                    dp[k as usize][j] + 1
                } else {
                    2
                };
                result = result.max(dp[j][i]);
            }
        }
        if result > 2 {
            result
        } else {
            0
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8])
    );
}

/*

[0, 2, 2, 2, 2, 2, 2, 2],
[0, 0, 3, 2, 2, 2, 2, 2],
[0, 0, 0, 3, 3, 2, 2, 2],
[0, 0, 0, 0, 3, 3, 3, 2],
[0, 0, 0, 0, 0, 3, 3, 3],
[0, 0, 0, 0, 0, 0, 3, 3],
[0, 0, 0, 0, 0, 0, 0, 3],
[0, 0, 0, 0, 0, 0, 0, 0]]

 */
