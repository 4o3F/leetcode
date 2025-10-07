use std::collections::BinaryHeap;

impl Solution {
    pub fn swim_in_water(mut g: Vec<Vec<i32>>) -> i32 {
        let (n, m, mut h) = (g.len(), g[0].len(), BinaryHeap::from([(-g[0][0], 0, 0)]));
        g[0][0] = -1;
        let mut r = 0;
        while let Some((t, y, x)) = h.pop() {
            r = r.max(-t);
            if (y, x) == (n - 1, m - 1) {
                break;
            }
            for (u, r) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if u < n && r < m && g[u][r] >= 0 {
                    h.push((-g[u][r], u, r));
                    g[u][r] = -1
                }
            }
        }
        r
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]))
}
