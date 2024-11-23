impl Solution {
    pub fn rotate_the_box(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let matrix = matrix
            .into_iter()
            .map(|row| {
                let (mut fallen_row, last_count) = row.iter().fold(
                    (Vec::<char>::new(), vec![0; 2]),
                    |(mut result, mut last_count), c| {
                        //
                        match c {
                            '#' => {
                                last_count[0] += 1;
                            }
                            '.' => {
                                last_count[1] += 1;
                            }
                            '*' => {
                                // Do push
                                (0..last_count[1]).for_each(|_| result.push('.'));
                                (0..last_count[0]).for_each(|_| result.push('#'));
                                last_count[0] = 0;
                                last_count[1] = 0;
                                result.push('*');
                            }
                            _ => unreachable!(),
                        }
                        (result, last_count)
                    },
                );
                (0..last_count[1]).for_each(|_| fallen_row.push('.'));
                (0..last_count[0]).for_each(|_| fallen_row.push('#'));
                fallen_row
            })
            .collect::<Vec<Vec<char>>>();
        let row = matrix.len();
        let col = matrix[0].len();
        let mut result = vec![vec!['.'; row]; col];
        for row_idx in 0..row {
            for col_idx in 0..col {
                result[col_idx][row - row_idx - 1] = matrix[row_idx][col_idx];
            }
        }
        result
    }
}

struct Solution {}
pub fn run() {
    let matrix = vec![
        vec!['#', '#', '*', '.', '*', '.'],
        vec!['#', '#', '#', '*', '.', '.'],
        vec!['#', '#', '#', '.', '#', '.'],
    ];
    tracing::info!("{:?}", Solution::rotate_the_box(matrix));
    let matrix = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
    tracing::info!("{:?}", Solution::rotate_the_box(matrix));
}
