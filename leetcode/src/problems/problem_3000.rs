impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .iter()
            .map(|d| (d[0] * d[0] + d[1] * d[1], d[0] * d[1]))
            .max()
            .unwrap()
            .1
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]])
    )
}
