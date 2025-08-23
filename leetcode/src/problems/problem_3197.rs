impl Solution {
    pub fn minimum_area(
        grid: &Vec<Vec<i32>>,
        st_i: usize,
        en_i: usize,
        st_j: usize,
        en_j: usize,
    ) -> i32 {
        let mut start_row = i32::MAX;
        let mut end_row = -1;
        let mut start_col = i32::MAX;
        let mut end_col = -1;
        let mut found = false;

        for i in st_i..=en_i {
            for j in st_j..=en_j {
                if grid[i][j] == 1 {
                    start_row = start_row.min(i as i32);
                    end_row = end_row.max(i as i32);
                    start_col = start_col.min(j as i32);
                    end_col = end_col.max(j as i32);
                    found = true;
                }
            }
        }

        if found {
            (end_row - start_row + 1) * (end_col - start_col + 1)
        } else {
            0
        }
    }

    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = i32::MAX;

        for i in 0..m {
            let one = Self::minimum_area(&grid, 0, i, 0, n - 1);
            for j in 0..n {
                let two = if i + 1 < m {
                    Self::minimum_area(&grid, i + 1, m - 1, 0, j)
                } else {
                    0
                };
                let three = if i + 1 < m && j + 1 < n {
                    Self::minimum_area(&grid, i + 1, m - 1, j + 1, n - 1)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        for j in 0..n {
            let one = Self::minimum_area(&grid, 0, m - 1, 0, j);
            for i in 0..m {
                let two = if j + 1 < n {
                    Self::minimum_area(&grid, 0, i, j + 1, n - 1)
                } else {
                    0
                };
                let three = if i + 1 < m && j + 1 < n {
                    Self::minimum_area(&grid, i + 1, m - 1, j + 1, n - 1)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        for j in (0..n).rev() {
            let one = if j + 1 < n {
                Self::minimum_area(&grid, 0, m - 1, j + 1, n - 1)
            } else {
                0
            };
            for i in 0..m {
                let two = Self::minimum_area(&grid, 0, i, 0, j);
                let three = if i + 1 < m {
                    Self::minimum_area(&grid, i + 1, m - 1, 0, j)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        for i in (0..m).rev() {
            let one = if i + 1 < m {
                Self::minimum_area(&grid, i + 1, m - 1, 0, n - 1)
            } else {
                0
            };
            for j in 0..n {
                let two = Self::minimum_area(&grid, 0, i, 0, j);
                let three = if j + 1 < n {
                    Self::minimum_area(&grid, 0, i, j + 1, n - 1)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        for i in 0..m {
            for j in i + 1..m {
                let one = Self::minimum_area(&grid, 0, i, 0, n - 1);
                let two = Self::minimum_area(&grid, i + 1, j, 0, n - 1);
                let three = if j + 1 < m {
                    Self::minimum_area(&grid, j + 1, m - 1, 0, n - 1)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        for i in 0..n {
            for j in i + 1..n {
                let one = Self::minimum_area(&grid, 0, m - 1, 0, i);
                let two = Self::minimum_area(&grid, 0, m - 1, i + 1, j);
                let three = if j + 1 < n {
                    Self::minimum_area(&grid, 0, m - 1, j + 1, n - 1)
                } else {
                    0
                };
                ans = ans.min(one + two + three);
            }
        }

        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]])
    )
}
