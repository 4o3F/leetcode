impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        use std::cmp;
        let mut x = 0i64;
        let mut y = i64::MIN;
        let k = k as i64;
        for &num in &nums {
            let num = num as i64;
            let tempx = cmp::max(x + num, y + (num ^ k));
            let tempy = cmp::max(num + y, x + (num ^ k));
            x = tempx;
            y = tempy;
        }
        x
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]])
    )
}
