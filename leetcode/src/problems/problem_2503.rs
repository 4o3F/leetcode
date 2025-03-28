use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_unstable_by_key(|x| x.1);

        let mut pq = BinaryHeap::<Reverse<(i32, (usize, usize))>>::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        pq.push(Reverse((grid[0][0], (0, 0))));
        visited[0][0] = true;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut results = Vec::<(usize, i32)>::new();
        for query in queries {
            let mut result = results.last().map_or(0, |x| x.1);
            loop {
                let (num, (x, y));
                match pq.peek() {
                    Some(d) => {
                        (num, (x, y)) = d.0;
                        if num >= query.1 {
                            // tracing::info!("{} >= {} break", num, query.1);
                            break;
                        }
                    }
                    None => break,
                }
                // tracing::info!("{}", num);
                pq.pop();
                result += 1;
                for (dx, dy) in directions.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                        continue;
                    }
                    if visited[nx as usize][ny as usize] {
                        continue;
                    }
                    visited[nx as usize][ny as usize] = true;
                    pq.push(Reverse((
                        grid[nx as usize][ny as usize],
                        (nx as usize, ny as usize),
                    )));
                }
            }
            results.push((query.0, result));
            // tracing::info!("{:?}", results);
        }

        results.sort_by_key(|x| x.0);

        results.into_iter().map(|x| x.1).collect()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::max_points(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2]
        )
    );
    tracing::info!(
        "{:?}",
        Solution::max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3])
    );
}
