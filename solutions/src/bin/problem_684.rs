impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut u: Vec<_> = (0..=edges.len()).collect();
        edges
            .into_iter()
            .find(|e| {
                let (a, b) = (e[0] as usize, e[1] as usize);
                while u[a] != u[u[a]] {
                    u[a] = u[u[a]]
                }
                while u[b] != u[u[b]] {
                    u[b] = u[u[b]]
                }
                let r = u[a] == u[b];
                let a = u[a];
                u[a] = u[b];
                r
            })
            .unwrap()
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
    )
}
