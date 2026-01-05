impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = Self::within_k(&edges1, k);
        if k > 0 {
            let d = Self::within_k(&edges2, k - 1).into_iter().max().unwrap();
            ans.iter_mut().for_each(|n| *n += d);
        }
        ans
    }
    fn within_k(edges: &[Vec<i32>], k: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut neis = vec![vec![]; n];
        for e in edges {
            neis[e[0] as usize].push(e[1]);
            neis[e[1] as usize].push(e[0]);
        }
        (0..n as i32).map(|i| Self::dfs(i, -1, k, &neis)).collect()
    }
    fn dfs(i: i32, prv: i32, k: i32, neis: &[Vec<i32>]) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut ans = 1;
        for &ne in &neis[i as usize] {
            if ne != prv {
                ans += Self::dfs(ne, i, k - 1, neis);
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::max_target_nodes(
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 7],
                vec![1, 4],
                vec![4, 5],
                vec![4, 6]
            ],
            2
        )
    )
}
