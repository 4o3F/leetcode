use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut costs = vec![vec![i32::MAX; n]; m];
        let mut result = i32::MAX;
        let mut heap = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();
        heap.push(Reverse((0, 0, 0)));
        while let Some(Reverse((cost, x, y))) = heap.pop() {
            if x == m - 1 && y == n - 1 {
                result = cost;
                break;
            }

            for (dx, dy) in directions.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let new_cost = cost + grid[nx][ny];
                if new_cost < costs[nx][ny] {
                    costs[nx][ny] = new_cost;
                    heap.push(Reverse((new_cost, nx, ny)));
                }
            }
        }
        result
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let matrix = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]];
    tracing::info!("{}", Solution::minimum_obstacles(matrix));
}
