impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent: Vec<i32> = (0..n).collect();
        let mut min_path_cost: Vec<i32> = vec![-1; n as usize];
        fn find_root(parent: &mut Vec<i32>, node: i32) -> i32 {
            if parent[node as usize] != node {
                parent[node as usize] = find_root(parent, parent[node as usize]);
            }
            parent[node as usize]
        }

        for edge in edges.iter() {
            let source = edge[0];
            let dest = edge[1];
            let cost = edge[2];
            let source_root = find_root(&mut parent, source);
            let dest_root = find_root(&mut parent, dest);

            min_path_cost[dest_root as usize] &= cost;

            if source_root != dest_root {
                min_path_cost[dest_root as usize] &= min_path_cost[source_root as usize];
                parent[source_root as usize] = dest_root;
            }
        }

        let mut result = Vec::new();
        for query in query.iter() {
            let start = query[0];
            let end = query[1];
            if start == end {
                result.push(0);
            } else if find_root(&mut parent, start) != find_root(&mut parent, end) {
                result.push(-1);
            } else {
                result.push(min_path_cost[find_root(&mut parent, start) as usize]);
            }
        }

        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]]
        )
    )
}
