use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; graph.len()];
        let (mut deg, mut safe) = (vec![0; g.len()], vec![false; g.len()]);
        for i in 0..g.len() {
            for &s in &graph[i] {
                let s = s as usize;
                g[s].push(i);
                deg[i] += 1
            }
        }
        let mut q = VecDeque::from_iter((0..g.len()).filter(|&i| deg[i] == 0));
        while let Some(i) = q.pop_front() {
            safe[i] = true;
            for &s in &g[i] {
                deg[s] -= 1;
                if deg[s] == 0 {
                    q.push_back(s)
                }
            }
        }
        (0..g.len())
            .filter(|&i| safe[i])
            .map(|i| i as i32)
            .collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ])
    );
}
