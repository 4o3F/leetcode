impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let (mut a, mut b) = (0, 0);
        for &v in grid[0].iter() {
            a += v as i64
        }
        (0..grid[0].len())
            .map(|x| {
                a -= grid[0][x] as i64;
                let m = a.max(b);
                b += grid[1][x] as i64;
                m
            })
            .min()
            .unwrap()
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]])
    );
}
