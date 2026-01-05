use std::collections::{HashSet, VecDeque};

type Matrix = (Vec<Vec<i32>>, (usize, usize), i32);
impl Solution {
    fn find_zero(matrix: &[Vec<i32>]) -> (usize, usize) {
        for row in 0..2 {
            for col in 0..3 {
                if matrix[row][col] == 0 {
                    return (row, col);
                }
            }
        }
        unreachable!()
    }
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut occur: HashSet<Vec<Vec<i32>>> = HashSet::new();
        let mut queue: VecDeque<Matrix> = VecDeque::new();
        let (row, col) = Self::find_zero(&board);
        queue.push_back((board.clone(), (row, col), 0));
        while !queue.is_empty() {
            let (matrix, (row, col), steps) = queue.pop_front().unwrap();
            if matrix == [[1, 2, 3], [4, 5, 0]] {
                return steps;
            }
            if occur.contains(&matrix) {
                continue;
            } else {
                occur.insert(matrix.clone());
            }
            // left
            if col > 0 {
                let mut new_matrix = matrix.clone();
                new_matrix[row].swap(col, col - 1);
                queue.push_back((new_matrix, (row, col - 1), steps + 1));
            }
            // right
            if col < 2 {
                let mut new_matrix = matrix.clone();
                new_matrix[row].swap(col, col + 1);
                queue.push_back((new_matrix, (row, col + 1), steps + 1));
            }
            // up
            if row > 0 {
                let mut new_matrix = matrix.clone();
                let tmp = new_matrix[row][col];
                new_matrix[row][col] = new_matrix[row - 1][col];
                new_matrix[row - 1][col] = tmp;
                queue.push_back((new_matrix, (row - 1, col), steps + 1));
            }
            // down
            if row < 1 {
                let mut new_matrix = matrix.clone();
                let tmp = new_matrix[row][col];
                new_matrix[row][col] = new_matrix[row + 1][col];
                new_matrix[row + 1][col] = tmp;
                queue.push_back((new_matrix, (row + 1, col), steps + 1));
            }
        }
        -1
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
    tracing::info!("{}", Solution::sliding_puzzle(board));
    let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
    tracing::info!("{}", Solution::sliding_puzzle(board));
}
