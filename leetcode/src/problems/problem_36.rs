use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::<u32>::new(); 9];
        let mut cols = vec![HashSet::<u32>::new(); 9];

        let mut grids = vec![vec![HashSet::<u32>::new(); 3]; 3];

        for row in 0..9 {
            let row_set = &mut rows[row];
            for col in 0..9 {
                let col_set = &mut cols[col];
                let grid = &mut grids[row / 3][col / 3];

                let num = match board[row][col] {
                    '.' => continue,
                    c => c.to_digit(10).unwrap(),
                };

                match row_set.contains(&num) {
                    true => return false,
                    false => (*row_set).insert(num),
                };

                match col_set.contains(&num) {
                    true => return false,
                    false => (*col_set).insert(num),
                };

                match grid.contains(&num) {
                    true => return false,
                    false => (*grid).insert(num),
                };
            }
        }
        true
    }
}

struct Solution;

pub fn run() {
    let sudoku = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    tracing::info!("{}", Solution::is_valid_sudoku(sudoku));
}
