use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, (entrance[0], entrance[1]))));
        maze[entrance[0] as usize][entrance[1] as usize] = '#';
        while !heap.is_empty() {
            tracing::info!("{:?}", heap);
            let (step, (x, y)) = heap.pop().unwrap().0;
            if (x != entrance[0] || y != entrance[1])
                && (x == 0
                    || x == (maze.len() - 1) as i32
                    || y == 0
                    || y == (maze[0].len() - 1) as i32)
            {
                return step;
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= maze.len() as i32 || ny < 0 || ny >= maze[0].len() as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if maze[nx][ny] == '.' {
                    heap.push(Reverse((step + 1, (nx as i32, ny as i32))));
                    maze[nx][ny] = '#';
                }
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::nearest_exit(
            [['+', '+', '+'], ['.', '.', '.'], ['+', '+', '+']]
                .into_iter()
                .map(|row| row.to_vec())
                .collect(),
            vec![1, 0]
        )
    )
}
