#[derive(Debug)]
enum State {
    Wall,
    Guard,
    Occupied,
    Unoccupied,
}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut field = Vec::<Vec<State>>::new();
        for _ in 0..m {
            let mut row = Vec::<State>::new();
            for _ in 0..n {
                row.push(State::Unoccupied);
            }
            field.push(row);
        }
        for wall in walls {
            let x = wall[0] as usize;
            let y = wall[1] as usize;
            field[x][y] = State::Wall;
        }

        for guard in &guards {
            let x = guard[0] as usize;
            let y = guard[1] as usize;
            field[x][y] = State::Guard;
        }

        // tracing::debug!("{:?}", field);
        for guard in guards {
            let row = guard[0] as usize;
            let col = guard[1] as usize;
            // tracing::debug!("guard {} {}", row, col);
            // left
            for col_idx in (0..col).rev() {
                match field[row][col_idx] {
                    State::Guard => break,
                    State::Wall => break,
                    _ => field[row][col_idx] = State::Occupied,
                }
            }

            // tracing::debug!("left {:?}", field);
            // right
            for col_idx in col + 1..(n as usize) {
                match field[row][col_idx] {
                    State::Guard => break,
                    State::Wall => break,
                    _ => field[row][col_idx] = State::Occupied,
                }
            }
            // tracing::debug!("right {:?}", field);
            // up
            for row_idx in (0..row).rev() {
                // tracing::debug!("y_idx up {}", row_idx);
                match field[row_idx][col] {
                    State::Guard => break,
                    State::Wall => break,
                    _ => field[row_idx][col] = State::Occupied,
                }
            }

            // tracing::debug!("up {:?}", field);
            // down
            for row_idx in row + 1..(m as usize) {
                // tracing::debug!("row_idx down {}", row_idx);
                match field[row_idx][col] {
                    State::Guard => break,
                    State::Wall => break,
                    _ => field[row_idx][col] = State::Occupied,
                }
            }
            // tracing::debug!("down {:?}", field);
        }
        // tracing::debug!("{:?}", field);

        field
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&state| matches!(state, State::Unoccupied))
            .count() as i32
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_unguarded(
            5,
            5,
            vec![vec![1, 4], vec![4, 1], vec![0, 3]],
            vec![vec![3, 2]]
        )
    );
    // return;
    tracing::info!(
        "{}",
        Solution::count_unguarded(
            2,
            7,
            vec![vec![1, 5], vec![1, 1], vec![1, 6], vec![0, 2]],
            vec![vec![0, 6], vec![0, 3], vec![0, 5]]
        )
    );
    tracing::info!(
        "{}",
        Solution::count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]]
        )
    );
}
