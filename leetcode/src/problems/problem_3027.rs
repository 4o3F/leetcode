impl Solution {
    pub fn number_of_pairs(mut p: Vec<Vec<i32>>) -> i32 {
        p.sort_by_key(|p| (-p[0], p[1]));
        let n = p.len();
        let mut ans = 0;
        for i in 0..n - 1 {
            let mut y: i32 = i32::MAX;
            let yi = p[i][1];
            for j in i + 1..n {
                let yj = p[j][1];
                if y > yj && yj >= yi {
                    ans += 1;
                    y = yj;
                    if yi == yj {
                        break;
                    }
                }
            }
        }
        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::number_of_pairs(vec![vec![1, 1], vec![2, 2], vec![3, 3]])
    )
}
