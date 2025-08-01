impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => Vec::new(),
            1 => Vec::from([Vec::from([1])]),
            2 => Vec::from([Vec::from([1]), Vec::from([1, 1])]),
            _ => {
                let mut result: Vec<Vec<i32>> = Vec::from([Vec::from([1]), Vec::from([1, 1])]);
                for row_idx in 3..num_rows + 1 {
                    tracing::info!("line {}", row_idx);
                    let mut line = Vec::new();
                    line.push(1);
                    for col_idx in 0..row_idx - 2 {
                        let col_idx = col_idx as usize;
                        match result.last() {
                            Some(last_row) => {
                                line.push(last_row[col_idx] + last_row[col_idx + 1]);
                            }
                            None => break,
                        }
                    }
                    if line.len() < row_idx as usize {
                        line.push(1);
                    }
                    result.push(line);
                }
                result
            }
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::generate(5))
}
