impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let width = grid[0].len();
        let height = grid.len();
        let mut grid = grid;
        for col in 1..width {
            let mut missed = 0;
            for row in 0..height {
                let num = grid[row][col];
                // tracing::info!(
                //     "Col {col} num {num} first {} second {} third {}",
                //     (row + 1 < height && grid[row + 1][col - 1] < num),
                //     (grid[row][col - 1] < num),
                //     (row >= 1 && grid[row - 1][col - 1] < num)
                // );
                if !((row + 1 < height && grid[row + 1][col - 1] < num)
                    || (grid[row][col - 1] < num)
                    || (row >= 1 && grid[row - 1][col - 1] < num))
                {
                    missed += 1;
                    grid[row][col] = i32::MAX;
                }
            }
            tracing::info!("Col {col} missed {missed}");
            if missed == height {
                // tracing::info!("col {} returned", col);
                return col as i32 - 1;
            }
        }

        width as i32 - 1
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let grid = vec![
        vec![2, 4, 3, 5],
        vec![5, 4, 9, 3],
        vec![3, 4, 2, 11],
        vec![10, 9, 13, 15],
    ];
    tracing::info!("{}", Solution::max_moves(grid));
    let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
    tracing::info!("{}", Solution::max_moves(grid));
    let grid = vec![
        vec![187, 167, 209, 251, 152, 236, 263, 128, 135],
        vec![267, 249, 251, 285, 73, 204, 70, 207, 74],
        vec![189, 159, 235, 66, 84, 89, 153, 111, 189],
        vec![120, 81, 210, 7, 2, 231, 92, 128, 218],
        vec![193, 131, 244, 293, 284, 175, 226, 205, 245],
    ];
    tracing::info!("{}", Solution::max_moves(grid));
}
