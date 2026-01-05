use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (total_row, total_col) = (move_time.len() as i32, move_time[0].len() as i32);
        let mut heap = BinaryHeap::<(Reverse<i32>, i32, i32, bool)>::new();
        let mut visited = vec![vec![false; total_col as usize]; total_row as usize];
        let directions = [[-1, 0], [0, -1], [1, 0], [0, 1]];
        heap.push((Reverse(0), 0, 0, false));

        while let Some((Reverse(current_time), current_row, current_col, last_is_one)) = heap.pop()
        {
            if current_row == total_row - 1 && current_col == total_col - 1 {
                return current_time;
            }
            for direction in directions {
                let (new_row, new_col) = (current_row + direction[0], current_col + direction[1]);
                if new_row < 0
                    || new_row >= total_row
                    || new_col < 0
                    || new_col >= total_col
                    || visited[new_row as usize][new_col as usize]
                {
                    continue;
                }
                visited[new_row as usize][new_col as usize] = true;

                heap.push((
                    Reverse(
                        current_time.max(move_time[new_row as usize][new_col as usize])
                            + if last_is_one { 2 } else { 1 },
                    ),
                    new_row,
                    new_col,
                    !last_is_one,
                ));
            }
        }

        unreachable!()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::min_time_to_reach(vec![
            vec![94, 79, 62, 27, 69, 84],
            vec![6, 32, 11, 82, 42, 30]
        ])
    )
}
