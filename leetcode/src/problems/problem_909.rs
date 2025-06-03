use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut idx_map: HashMap<i32, (usize, usize)> = HashMap::new();

        let (mut flip, mut current_idx) = (false, 1);
        for row in (0..n).rev() {
            // tracing::info!("row {} {}", row, flip);
            if flip {
                for col in (0..n).rev() {
                    // tracing::info!("{} {} {}", row, col, current_idx);
                    idx_map.insert(current_idx, (row, col));
                    current_idx += 1;
                }
            } else {
                for col in 0..n {
                    // tracing::info!("{} {} {}", row, col, current_idx);
                    idx_map.insert(current_idx, (row, col));
                    current_idx += 1;
                }
            }
            flip = !flip;
        }

        // tracing::info!("{:?}", idx_map);

        // current_idx current_steps
        let mut queue = VecDeque::<(i32, i32)>::new();
        let mut visited = vec![false; n * n];
        queue.push_back((1, 0));
        while !queue.is_empty() {
            let (current_idx, current_steps) = queue.pop_front().unwrap();
            // tracing::info!("current {} {}", current_idx, current_steps);
            if current_idx as usize == n * n {
                return current_steps;
            }
            for next_idx in (current_idx + 1)..=(current_idx + 6).min((n * n) as i32) {
                let mut next_idx = next_idx;
                let (next_row, next_col) = idx_map.get(&next_idx).unwrap();
                if board[*next_row][*next_col] != -1 {
                    // take snake or ladder
                    next_idx = board[*next_row][*next_col];
                }

                if !visited[next_idx as usize - 1] {
                    // tracing::info!("added {}", next_idx);
                    queue.push_back((next_idx, current_steps + 1));
                    visited[next_idx as usize - 1] = true;
                }
            }
        }
        -1
    }
}

struct Solution;

pub fn run() {
    let board = vec![
        [-1, -1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1, -1],
        [-1, 35, -1, -1, 13, -1],
        [-1, -1, -1, -1, -1, -1],
        [-1, 15, -1, -1, -1, -1],
    ]
    .into_iter()
    .map(|x| x.to_vec())
    .collect::<Vec<_>>();
    tracing::info!("{}", Solution::snakes_and_ladders(board))
}
