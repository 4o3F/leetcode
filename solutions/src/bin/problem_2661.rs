use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut map = HashMap::<i32, (usize, usize)>::new();
        for i in 0..m {
            for j in 0..n {
                *map.entry(mat[i][j]).or_insert((m, n)) = (i, j);
            }
        }
        let mut row = vec![0; m];
        let mut col = vec![0; n];
        for (idx, num) in arr.iter().enumerate() {
            let (i, j) = map[num];
            row[i] += 1;
            col[j] += 1;
            if col[j] == m || row[i] == n {
                return idx as i32;
            }
        }
        unreachable!()
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]])
    )
}
