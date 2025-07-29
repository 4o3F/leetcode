impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut tree = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            tree[u].push(v);
            tree[v].push(u);
        }

        let mut subtree_xor = vec![0; n];
        let mut in_time = vec![0; n];
        let mut out_time = vec![0; n];
        let mut time = 0;

        fn dfs(
            node: usize,
            parent: usize,
            tree: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            subtree_xor: &mut Vec<i32>,
            in_time: &mut Vec<i32>,
            out_time: &mut Vec<i32>,
            time: &mut i32,
        ) {
            subtree_xor[node] = nums[node];
            in_time[node] = *time;
            *time += 1;

            for &nei in &tree[node] {
                if nei != parent {
                    dfs(nei, node, tree, nums, subtree_xor, in_time, out_time, time);
                    subtree_xor[node] ^= subtree_xor[nei];
                }
            }

            out_time[node] = *time;
            *time += 1;
        }

        dfs(
            0,
            n,
            &tree,
            &nums,
            &mut subtree_xor,
            &mut in_time,
            &mut out_time,
            &mut time,
        );

        fn is_ancestor(u: usize, v: usize, in_time: &Vec<i32>, out_time: &Vec<i32>) -> bool {
            in_time[u] <= in_time[v] && out_time[v] <= out_time[u]
        }

        let total_xor = subtree_xor[0];
        let mut directed_edges = vec![];

        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if is_ancestor(u, v, &in_time, &out_time) {
                directed_edges.push((v, u));
            } else {
                directed_edges.push((u, v));
            }
        }

        let mut res = i32::MAX;

        for i in 0..directed_edges.len() {
            let a = directed_edges[i].0;
            for j in i + 1..directed_edges.len() {
                let b = directed_edges[j].0;
                let (x, y, z) = if is_ancestor(a, b, &in_time, &out_time) {
                    (
                        subtree_xor[b],
                        subtree_xor[a] ^ subtree_xor[b],
                        total_xor ^ subtree_xor[a],
                    )
                } else if is_ancestor(b, a, &in_time, &out_time) {
                    (
                        subtree_xor[a],
                        subtree_xor[b] ^ subtree_xor[a],
                        total_xor ^ subtree_xor[b],
                    )
                } else {
                    (
                        subtree_xor[a],
                        subtree_xor[b],
                        total_xor ^ subtree_xor[a] ^ subtree_xor[b],
                    )
                };

                let max_val = *[x, y, z].iter().max().unwrap();
                let min_val = *[x, y, z].iter().min().unwrap();
                res = res.min(max_val - min_val);
            }
        }

        res
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::minimum_score(
            vec![1, 5, 5, 4, 11],
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
        )
    )
}
