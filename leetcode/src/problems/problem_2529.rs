impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let (mut pos, mut neg) = (nums.len(), 0);
        for (idx, &num) in nums.iter().enumerate() {
            if num > 0 {
                pos = pos.min(idx);
            } else if num < 0 {
                neg = neg.max(idx + 1);
            }
        }
        // tracing::info!("pos: {}, neg: {}", pos, neg);
        (nums.len() - pos).max(neg) as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    tracing::info!("{}", Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    tracing::info!("{}", Solution::maximum_count(vec![5, 20, 66, 1314]));
    tracing::info!("{}", Solution::maximum_count(vec![-1314, -66, -20, -5]));
}
