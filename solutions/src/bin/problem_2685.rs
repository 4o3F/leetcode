struct UnionFind {
    parents: Vec<usize>,
    nodes: Vec<usize>,
    edges: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
            nodes: vec![1; size],
            edges: vec![0; size],
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parents[node] == node {
            node
        } else {
            self.parents[node] = self.find(self.parents[node]);
            self.parents[node]
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root == b_root {
            self.edges[a_root] += 1;
            return;
        }

        self.parents[a_root] = b_root;
        self.nodes[b_root] += self.nodes[a_root];
        self.edges[b_root] += self.edges[a_root] + 1;
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            uf.union(a, b);
        }

        let mut result = 0;
        for i in 0..n as usize {
            // tracing::info!("{} {} {} {}", i, uf.parents[i], uf.nodes[i], uf.edges[i]);
            if uf.parents[i] == i && uf.edges[i] == uf.nodes[i] * (uf.nodes[i] - 1) / 2 {
                result += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
        )
    );
}
