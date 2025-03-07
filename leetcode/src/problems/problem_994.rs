use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::<Reverse<(i32, (i32, i32))>>::new();
        let (mut total_orange, mut rotten_orange) = (0, 0);
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 2 {
                    heap.push(Reverse((0, (row as i32, col as i32))));
                    total_orange += 1;
                    rotten_orange += 1;
                } else if grid[row][col] == 1 {
                    total_orange += 1;
                }
            }
        }

        let mut time = 0;
        while !heap.is_empty() {
            // tracing::info!("{:?}", heap);
            let (t, (row, col)) = heap.pop().unwrap().0;
            if t > time {
                time = t;
            }

            for (i, j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nrow, ncol) = (row + i, col + j);
                if nrow < 0 || nrow >= grid.len() as i32 || ncol < 0 || ncol >= grid[0].len() as i32
                {
                    continue;
                }
                if grid[nrow as usize][ncol as usize] == 1 {
                    grid[nrow as usize][ncol as usize] = 2;
                    heap.push(Reverse((time + 1, (nrow, ncol))));
                    rotten_orange += 1;
                }
            }
        }

        if total_orange == rotten_orange {
            time
        } else {
            -1
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::oranges_rotting(
            [[2, 1, 1], [1, 1, 0], [0, 1, 1]]
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
        ),
    );
    tracing::info!(
        "{}",
        Solution::oranges_rotting(
            [[2, 1, 1], [0, 1, 1], [1, 0, 1]]
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
        ),
    );
}
