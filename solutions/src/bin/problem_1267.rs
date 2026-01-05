impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let (mut row_count, mut col_count, mut coord) = (vec![0; row], vec![0; col], vec![]);
        for row_idx in 0..row {
            for col_idx in 0..col {
                if grid[row_idx][col_idx] == 1 {
                    row_count[row_idx] += 1;
                    col_count[col_idx] += 1;
                    coord.push((row_idx, col_idx));
                }
            }
        }
        coord
            .into_iter()
            .filter(|&(row_idx, col_idx)| row_count[row_idx] > 1 || col_count[col_idx] > 1)
            .count() as i32
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_servers(vec![vec![1, 0], vec![0, 1]]));
    tracing::info!(
        "{}",
        Solution::count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ])
    );
}
