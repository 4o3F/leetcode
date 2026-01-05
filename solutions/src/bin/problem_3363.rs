impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut ans = 0;

        // Diagonal sum (child at (0,0))
        for (i, fruit) in fruits.iter().enumerate() {
            ans += fruit[i];
        }

        let dp = |fruits: &Vec<Vec<i32>>| -> i32 {
            let mut prev = vec![i32::MIN; n];
            let mut curr = vec![i32::MIN; n];

            prev[n - 1] = fruits[0][n - 1];

            for (i, fruit) in fruits.iter().enumerate() {
                let start = std::cmp::max(n - 1 - i, i + 1);
                for j in start..n {
                    let mut best = prev[j];
                    if j >= 1 {
                        best = best.max(prev[j - 1]);
                    }
                    if j + 1 < n {
                        best = best.max(prev[j + 1]);
                    }
                    curr[j] = best + fruit[j];
                }
                std::mem::swap(&mut prev, &mut curr);
            }

            prev[n - 1]
        };

        ans += dp(&fruits);

        // Transpose the matrix (flip along main diagonal)
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..i {
                let tmp = fruits[i][j];
                fruits[i][j] = fruits[j][i];
                fruits[j][i] = tmp;
            }
        }

        ans += dp(&fruits);
        ans
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_collected_fruits(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 8, 7],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ])
    )
}
