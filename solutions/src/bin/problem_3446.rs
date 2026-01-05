impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (total_row, total_col) = (grid.len(), grid[0].len());
        let mut starts = Vec::<(usize, usize)>::new();
        for row in 0..total_row {
            starts.push((row, 0));
        }
        for col in 1..total_col {
            starts.push((0, col));
        }

        for (start_row, start_col) in starts {
            // tracing::info!("new start row {} col {}", start_row, start_col);
            let mut diagonals = Vec::<i32>::new();
            let mut step = 0;
            while start_row + step < total_row && start_col + step < total_col {
                diagonals.push(grid[start_row + step][start_col + step]);
                // tracing::info!(
                //     "row {} col {} number {}",
                //     start_row + step,
                //     start_col + step,
                //     grid[start_row + step][start_col + step]
                // );
                step += 1;
            }
            diagonals.sort();
            step = 0;
            while start_row + step < total_row && start_col + step < total_col {
                if start_col == 0 {
                    grid[start_row + step][start_col + step] =
                        diagonals[diagonals.len() - step - 1];
                } else if start_row == 0 {
                    grid[start_row + step][start_col + step] = diagonals[step];
                } else {
                    unreachable!()
                }
                step += 1;
            }
            // tracing::info!("{:#?}", grid);
        }
        grid
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:#?}",
        Solution::sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]])
    )
}
