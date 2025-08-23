impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let (mut result, mut heights) = (0, vec![0; mat[0].len()]);
        for row in &mat {
            for col_idx in 0..heights.len() {
                let mut col = row[col_idx] * (1 + heights[col_idx]);
                heights[col_idx] = col;
                for j in (0..=col_idx).rev() {
                    col = col.min(heights[j]);
                    result += col
                }
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
    )
}
