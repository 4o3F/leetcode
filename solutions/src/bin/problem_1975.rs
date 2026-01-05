impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0i64;
        let (mut min_negative, mut negative_count) = (i32::MAX, 0);
        matrix.into_iter().flatten().for_each(|num| {
            if num <= 0 {
                negative_count += 1;
            }
            min_negative = min_negative.min(num.abs());
            sum += num.abs() as i64;
        });

        tracing::debug!(
            "sum: {}, min_negative: {}, negative_count: {}",
            sum,
            min_negative,
            negative_count
        );
        match negative_count % 2 {
            0 => sum,
            1 => sum - (min_negative as i64) * 2,
            _ => unreachable!(),
        }
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let matrix = vec![vec![1, -1], vec![-1, 1]];
    tracing::info!("{}", Solution::max_matrix_sum(matrix));
    let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
    tracing::info!("{}", Solution::max_matrix_sum(matrix));
    let matrix = vec![vec![-1, 0, -1], vec![-2, 1, 3], vec![3, 2, 2]];
    tracing::info!("{}", Solution::max_matrix_sum(matrix));
    let matrix = vec![vec![2, 9, 3], vec![5, 4, -4], vec![1, 7, 1]];
    tracing::info!("{}", Solution::max_matrix_sum(matrix));
}
