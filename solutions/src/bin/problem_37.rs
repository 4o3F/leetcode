impl Solution {
    fn solve(
        board: &mut [[u8; 9]; 9],
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        grids: &mut [[u16; 3]; 3],
        empties: &mut Vec<(usize, usize)>,
    ) -> bool {
        if empties.is_empty() {
            return true;
        }

        let (idx, _, _, mask) = empties
            .iter()
            .enumerate()
            .map(|(idx, &(row, col))| {
                let mask = !(rows[row] | cols[col] | grids[row / 3][col / 3]) & 0x1FF;
                (idx, row, col, mask)
            })
            .min_by_key(|&(_, _, _, mask)| mask.count_ones())
            .unwrap();

        let (r, c) = empties.swap_remove(idx);
        let mut bits = mask;

        while bits > 0 {
            let bit = bits & (!bits + 1);
            let num = bit.trailing_zeros() as u8 + 1;

            board[r][c] = num;
            rows[r] |= bit;
            cols[c] |= bit;
            grids[r / 3][c / 3] |= bit;

            if Self::solve(board, rows, cols, grids, empties) {
                return true;
            }

            board[r][c] = 0;
            rows[r] &= !bit;
            cols[c] &= !bit;
            grids[r / 3][c / 3] &= !bit;

            bits &= bits - 1;
        }

        empties.push((r, c));
        false
    }

    pub fn solve_sudoku(board: &mut [Vec<char>]) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut grids = [[0u16; 3]; 3];

        let mut sudoku = [[0u8; 9]; 9];
        let mut empties = Vec::new();

        for row in 0..9 {
            for col in 0..9 {
                match board[row][col] {
                    '.' => {
                        sudoku[row][col] = 0;
                        empties.push((row, col));
                    }
                    ch => {
                        let num = ch.to_digit(10).unwrap() as u8;
                        sudoku[row][col] = num;
                        let bit = 1 << (num - 1);
                        rows[row] |= bit;
                        cols[col] |= bit;
                        grids[row / 3][col / 3] |= bit;
                    }
                }
            }
        }

        Self::solve(&mut sudoku, &mut rows, &mut cols, &mut grids, &mut empties);

        for r in 0..9 {
            for c in 0..9 {
                board[r][c] = (sudoku[r][c] + b'0') as char;
            }
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut sudoku = vec![
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
    Solution::solve_sudoku(&mut sudoku);
    println!("{:?}", sudoku);
}
