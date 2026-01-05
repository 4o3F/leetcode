use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (rows, cols) = (grid.len(), grid[0].len());
        let mut queue = BinaryHeap::new();
        let mut visited = vec![vec![false; cols]; rows];
        queue.push(Reverse((0, 0, 0)));
        visited[0][0] = true;

        while let Some(Reverse((time, row, col))) = queue.pop() {
            for (drow, dcol) in directions {
                let (nrow, ncol) = (row as i32 + drow, col as i32 + dcol);
                if nrow < 0 || nrow >= rows as i32 || ncol < 0 || ncol >= cols as i32 {
                    continue;
                }
                let (nrow, ncol) = (nrow as usize, ncol as usize);
                if visited[nrow][ncol] {
                    continue;
                }
                let mut ntime = time + 1;
                let cell_value = grid[nrow][ncol];
                if cell_value > ntime {
                    ntime += ((cell_value - time) / 2) * 2;
                }
                if nrow == rows - 1 && ncol == cols - 1 {
                    return ntime;
                }

                visited[nrow][ncol] = true;
                queue.push(Reverse((ntime, nrow, ncol)));
            }
        }

        unreachable!()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let grid = vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]];
    tracing::info!("{}", Solution::minimum_time(grid));
}
