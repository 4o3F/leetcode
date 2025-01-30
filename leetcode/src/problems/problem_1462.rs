impl Solution {
    pub fn check_if_prerequisite(n: i32, pre: Vec<Vec<i32>>, q: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut p = vec![vec![0; n]; n];
        for e in pre {
            p[e[1] as usize][e[0] as usize] = 1
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if p[i][k] * p[k][j] > 0 {
                        p[i][j] = 1
                    }
                }
            }
        }
        q.iter()
            .map(|e| p[e[1] as usize][e[0] as usize] > 0)
            .collect()
    }
}

struct Solution;
pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::check_if_prerequisite(2, vec![vec![0, 1]], vec![vec![1, 0]])
    );
}
