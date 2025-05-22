use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (rows, cols): (HashSet<_>, HashSet<_>) = (0..matrix.len())
            .flat_map(|row| (0..matrix[0].len()).map(move |col| (row, col)))
            .filter(|(row, col)| matrix[*row][*col] == 0)
            .collect();

        for row in rows {
            matrix[row].iter_mut().for_each(|cell| *cell = 0);
        }

        for col in cols {
            matrix.iter_mut().for_each(|row| row[col] = 0);
        }
    }
}

struct Solution;

pub fn run() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    tracing::info!("{:?}", matrix);
}
