use std::collections::VecDeque;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        mut values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let (mut cnt, mut g, mut deg) = (0, vec![vec![]; n as usize], vec![0; n as usize]);
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            deg[u] += 1;
            deg[v] += 1;
            g[u].push(v);
            g[v].push(u)
        }
        let mut q = VecDeque::from_iter((0..n as usize).filter(|&u| deg[u] < 2));
        while let Some(u) = q.pop_front() {
            deg[u] -= 1;
            if values[u] % k == 0 {
                cnt += 1
            }
            for &v in &g[u] {
                if deg[v] < 1 {
                    continue;
                }
                deg[v] -= 1;
                values[v] += values[u] % k;
                if deg[v] == 1 {
                    q.push_back(v);
                }
            }
        }
        cnt
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_k_divisible_components(
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6
        )
    )
}
