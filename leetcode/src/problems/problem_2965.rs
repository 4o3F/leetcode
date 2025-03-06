impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![0; grid.len() * grid[0].len()];
        let total_num = (grid.len() * grid[0].len()) as i32;
        let (mut twice, mut total) = (0, 0);
        for num in grid.iter().flatten() {
            count[*num as usize - 1] += 1;
            total += num;
            if count[*num as usize - 1] == 2 {
                twice = *num;
            }
        }
        let missing =
            (1 + total_num) * total_num / 2 - (total - twice);
        vec![twice, missing]
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::find_missing_and_repeated_values(vec![vec![9,1,7],vec![8,9,2],vec![3,4,6]])
    )
}
