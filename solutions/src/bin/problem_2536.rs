impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut data = vec![vec![0; n + 1]; n];

        for q in queries {
            let (x, y, u, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            for i in x..=u {
                data[i][y] += 1;
                data[i][v + 1] -= 1;
            }
        }

        let mut ret = vec![];
        for i in 0..n {
            let mut v = vec![];
            let mut sum = 0;
            for j in 0..n {
                sum += data[i][j];
                v.push(sum);
            }
            ret.push(v);
        }

        ret
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]])
    );
}
