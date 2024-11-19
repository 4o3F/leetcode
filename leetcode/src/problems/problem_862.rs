use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut queue = VecDeque::from([(0, -1)]);
        let (mut sum, mut res) = (0i64, i32::MAX);
        for (i, &n) in nums.iter().enumerate() {
            sum += n as i64;
            while queue.front().is_some_and(|f| sum - f.0 >= k as i64) {
                res = res.min(i as i32 - queue.pop_front().unwrap().1);
            }
            tracing::debug!("Iter {i} sum: {sum} queue: {queue:?} res: {res}");
            while queue.back().is_some_and(|b| b.0 >= sum) {
                queue.pop_back();
            }
            tracing::debug!("Iter {i} sum: {sum} queue: {queue:?} res: {res}");
            queue.push_back((sum, i as i32));
        }
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::shortest_subarray(vec![84, -37, 32, 40, 95], 167)
    );
}
