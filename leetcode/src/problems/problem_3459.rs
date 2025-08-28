impl Solution {
    const DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut memo = vec![vec![vec![0; 1 << 3]; n]; m];
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                let maxs = [m - i, j + 1, i + 1, n - j];
                for k in 0..4 {
                    if maxs[k] > ans {
                        ans = ans
                            .max((Self::dfs(i, j, k as i32, 1, 2, &grid, &mut memo) + 1) as usize);
                    }
                }
            }
        }
        ans as i32
    }

    fn dfs(
        i: usize,
        j: usize,
        k: i32,
        can_turn: i32,
        target: i32,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let ni = i as i32 + Self::DIRS[k as usize].0;
        let nj = j as i32 + Self::DIRS[k as usize].1;

        if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 {
            return 0;
        }
        let (ni, nj) = (ni as usize, nj as usize);
        if grid[ni][nj] != target {
            return 0;
        }

        let mask = (k << 1 | can_turn) as usize;
        if memo[ni][nj][mask] > 0 {
            return memo[ni][nj][mask];
        }

        let mut res = Self::dfs(ni, nj, k, can_turn, 2 - target, grid, memo);
        if can_turn == 1 {
            let maxs = [m - ni - 1, nj, ni, n - nj - 1];
            let nk = (k + 1) % 4;
            if maxs[nk as usize] > res as usize {
                res = res.max(Self::dfs(ni, nj, nk as i32, 0, 2 - target, grid, memo));
            }
        }
        memo[ni][nj][mask] = res + 1;
        res + 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::len_of_v_diagonal(vec![
            vec![2, 2, 1, 2, 2],
            vec![2, 0, 2, 2, 0],
            vec![2, 0, 1, 1, 0],
            vec![1, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 2]
        ])
    )
}
