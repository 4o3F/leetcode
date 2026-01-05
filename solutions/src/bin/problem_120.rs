impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 1 {
            return triangle[0][0];
        }
        let mut sum_triangle = triangle.clone();
        let mut last_line = sum_triangle[0].clone();
        for line in 1..triangle.len() {
            let current_line = &mut sum_triangle[line];
            for col_idx in 0..current_line.len() {
                current_line[col_idx] += last_line
                    .get(if col_idx == 0 { usize::MAX } else { col_idx } - 1)
                    .unwrap_or(&i32::MAX)
                    .min(last_line.get(col_idx).unwrap_or(&i32::MAX));
            }
            last_line = current_line.clone();
        }

        // tracing::info!("{:#?}", sum_triangle);

        *sum_triangle.last().unwrap().iter().min().unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    )
}
