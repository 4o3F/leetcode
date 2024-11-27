use std::{collections::BinaryHeap, i32};

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: i32,
    to: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from.cmp(&other.from)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.from.partial_cmp(&self.from)
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Solution {
    fn find_shortest_path(n: i32, mut paths: BinaryHeap<Edge>) -> i32 {
        let mut steps: Vec<i32> = vec![i32::MAX; n as usize];
        steps[0] = 0;
        while !paths.is_empty() {
            let edge = paths.pop().unwrap();
            tracing::debug!("{:?}", edge);
            steps[edge.to as usize] = steps[edge.to as usize].min(steps[edge.from as usize] + 1);
            // if edge.to == n - 1 {
            //     break;
            // }
        }
        steps[n as usize - 1]
    }

    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut paths: BinaryHeap<Edge> = BinaryHeap::new();
        (0..n - 1).for_each(|from| paths.push(Edge { from, to: from + 1 }));
        queries
            .into_iter()
            .fold(vec![], |mut result: Vec<i32>, query| {
                paths.push(Edge {
                    from: query[0],
                    to: query[1],
                });
                result.push(Self::find_shortest_path(n, paths.clone()));
                result
            })
    }
}

struct Solution {}
pub fn run() {
    let queries = vec![
        vec![0, 13],
        vec![10, 14],
        vec![3, 9],
        vec![4, 8],
        vec![7, 12],
    ];
    tracing::info!(
        "{:?}",
        Solution::shortest_distance_after_queries(15, queries)
    );
    let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
    tracing::info!(
        "{:?}",
        Solution::shortest_distance_after_queries(5, queries)
    );
}
