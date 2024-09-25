use std::collections::VecDeque;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut is_connected = is_connected;
        let n = is_connected.len();

        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        let mut result = 0;

        for row in 0..n {
            if visited[row] {
                continue;
            }

            for col in 0..n {
                if is_connected[row][col] == 1 && !visited[col] {
                    is_connected[row][col] = 0;
                    queue.push_back(col);
                }
            }

            visited[row] = true;

            if !queue.is_empty() {
                result += 1;
            }

            while let Some(col) = queue.pop_front() {
                if visited[col] {
                    continue;
                }
                for row in 0..n {
                    if is_connected[col][row] == 1 && !visited[row] {
                        queue.push_back(row);
                        is_connected[col][row] = 0;
                    }
                }

                visited[col] = true;
            }
        }

        result
    }
}

struct Solution {}

pub fn run() {
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    tracing::info!("{}", Solution::find_circle_num(is_connected));
}
