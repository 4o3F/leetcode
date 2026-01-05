use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut row_chunks = Vec::<Vec<i32>>::new();
        for row_idx in 0..row {
            let row = &matrix[row_idx];
            let mut last_occur: Option<i32> = None;
            let mut count = 0;
            let mut chunk = Vec::<i32>::new();
            for col_idx in 0..col {
                match row[col_idx] {
                    0 => {
                        if let Some(current_last_occur) = last_occur {
                            if current_last_occur == 0 {
                                count += 1;
                            } else {
                                chunk.push(count);
                                last_occur = Some(0);
                                count = 1;
                            }
                        } else {
                            last_occur = Some(0);
                            count = 1;
                        }
                    }
                    1 => {
                        if let Some(current_last_occur) = last_occur {
                            if current_last_occur == 1 {
                                count += 1;
                            } else {
                                chunk.push(count);
                                last_occur = Some(1);
                                count = 1;
                            }
                        } else {
                            last_occur = Some(1);
                            count = 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            chunk.push(count);
            row_chunks.push(chunk);
        }
        let mut frequency = HashMap::<Vec<i32>, i32>::new();
        // tracing::debug!("{:?}", row_chunks);
        row_chunks.into_iter().for_each(|chunk| {
            *frequency.entry(chunk).or_insert(0) += 1;
        });
        // tracing::debug!("{:?}", frequency);
        frequency
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .1
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    let arr = vec![
        vec![1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
    ];
    tracing::info!("{}", Solution::max_equal_rows_after_flips(arr));
    let arr = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    tracing::info!("{}", Solution::max_equal_rows_after_flips(arr));
}
