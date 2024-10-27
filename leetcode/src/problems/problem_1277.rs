impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (matrix.len(), matrix[0].len());
        let mut dp = matrix.clone();
        for x in 1..height {
            for y in 1..width {
                if matrix[x][y] == 1 {
                    dp[x][y] = 1 + dp[x - 1][y - 1].min(dp[x - 1][y]).min(dp[x][y - 1]);
                }
            }
        }
        dp.into_iter().flatten().sum()
    }
}

struct Solution {}

pub fn run() {
    let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
    tracing::info!("{}", Solution::count_squares(matrix));
}
