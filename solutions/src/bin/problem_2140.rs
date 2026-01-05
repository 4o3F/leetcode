impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; questions.len()];
        for (idx, question) in questions.iter().enumerate().rev() {
            dp[idx] = (question[0] as i64 + *dp.get(idx + question[1] as usize + 1).unwrap_or(&0))
                .max(*dp.get(idx + 1).unwrap_or(&0))
        }

        dp[0]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
    );
}
