impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut min_row, mut max_row, mut min_col, mut max_col) =
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN);

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    min_row = min_row.min(row as i32);
                    max_row = max_row.max(row as i32);
                    min_col = min_col.min(col as i32);
                    max_col = max_col.max(col as i32);
                }
            }
        }

        (max_row - min_row + 1) * (max_col - min_col + 1)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::minimum_area(vec![vec![0, 1, 0], vec![1, 0, 1]])
    );
}
