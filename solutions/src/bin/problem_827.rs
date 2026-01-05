use std::collections::HashMap;

fn paint(grid: &mut [Vec<i32>], paint_id: i32, i: usize, j: usize) -> i32 {
    grid[i][j] = paint_id;

    let mut res = 1;

    if i > 0 && grid[i - 1][j] == 1 {
        res += paint(grid, paint_id, i - 1, j);
    }

    if j > 0 && grid[i][j - 1] == 1 {
        res += paint(grid, paint_id, i, j - 1);
    }

    if i < grid.len() - 1 && grid[i + 1][j] == 1 {
        res += paint(grid, paint_id, i + 1, j);
    }

    if j < grid[0].len() - 1 && grid[i][j + 1] == 1 {
        res += paint(grid, paint_id, i, j + 1);
    }

    res
}

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut paint_map = HashMap::new();
        let mut paint_id = 2;
        let mut has_zeros = false;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    paint_map.insert(paint_id, paint(&mut grid, paint_id, i, j));
                    paint_id += 1;
                } else if grid[i][j] == 0 {
                    has_zeros = true;
                }
            }
        }

        if !has_zeros {
            return (grid.len() * grid[0].len()) as i32;
        }

        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    let mut pot = 0;
                    let mut curs = vec![];

                    if i > 0 && grid[i - 1][j] > 1 {
                        curs.push(grid[i - 1][j]);
                        pot += paint_map.get(&grid[i - 1][j]).copied().unwrap();
                    }

                    if j > 0 && grid[i][j - 1] > 1 && !curs.contains(&grid[i][j - 1]) {
                        curs.push(grid[i][j - 1]);
                        pot += paint_map.get(&grid[i][j - 1]).copied().unwrap();
                    }

                    if i < grid.len() - 1 && grid[i + 1][j] > 1 && !curs.contains(&grid[i + 1][j]) {
                        curs.push(grid[i + 1][j]);
                        pot += paint_map.get(&grid[i + 1][j]).copied().unwrap();
                    }

                    if j < grid[0].len() - 1
                        && grid[i][j + 1] > 1
                        && !curs.contains(&grid[i][j + 1])
                    {
                        curs.push(grid[i][j + 1]);
                        pot += paint_map.get(&grid[i][j + 1]).copied().unwrap();
                    }

                    res = i32::max(res, pot + 1);
                }
            }
        }

        res
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::largest_island(vec![vec![1, 0], vec![0, 1]]))
}
