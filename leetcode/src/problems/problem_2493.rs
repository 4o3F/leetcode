use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = (n + 1) as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            g[a].push(b);
            g[b].push(a)
        }
        let mut group = vec![0; n];
        let mut gs = vec![0];
        for start in 1..n {
            if group[start] < 1 {
                gs.push(0)
            };
            let mut color = vec![0; n];
            let mut q = VecDeque::from([start]);
            color[start] = 1;
            let mut lvl = 0;
            while q.len() > 0 {
                for _ in 0..q.len() {
                    let u = q.pop_front().unwrap();
                    if group[u] < 1 {
                        group[u] = gs.len() - 1
                    }
                    for &v in &g[u] {
                        if color[v] < 1 {
                            color[v] = 3 - color[u];
                            q.push_back(v)
                        } else if color[v] == color[u] {
                            return -1;
                        }
                    }
                }
                lvl += 1;
            }
            gs[group[start]] = lvl.max(gs[group[start]])
        }
        gs.iter().sum::<usize>() as i32
    }
}

struct Solution;

pub fn run() {
    let v = vec![[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]].into_iter().map(|x| x.to_vec()).collect::<Vec<_>>();
    tracing::info!(
        "{:?}",
        Solution::magnificent_sets(6, v)
    )
}
