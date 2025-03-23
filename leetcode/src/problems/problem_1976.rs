use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];

        for road in roads {
            let (src, dst, cost) = (road[0] as usize, road[1] as usize, road[2] as i64);
            graph[src].push((-cost, dst));
            graph[dst].push((-cost, src));
        }

        let (mut time_spent, mut count) = (vec![i64::MIN; graph.len()], vec![1; graph.len()]);
        let mut queue = BinaryHeap::from([(0, 0)]);
        time_spent[0] = 0;
        while let Some((time, node)) = queue.pop() {
            for &(next_time, next_node) in &graph[node] {
                if time + next_time > time_spent[next_node] {
                    count[next_node] = count[node];
                    time_spent[next_node] = time + next_time;
                    queue.push((time_spent[next_node], next_node));
                } else if time + next_time == time_spent[next_node] {
                    count[next_node] = (count[node] + count[next_node]) % 1_000_000_007;
                }
            }
        }
        count[n as usize - 1]
    }
}

struct Solution;

pub fn run() {
    let roads = [
        [0, 6, 7],
        [0, 1, 2],
        [1, 2, 3],
        [1, 3, 3],
        [6, 3, 3],
        [3, 5, 1],
        [6, 5, 1],
        [2, 5, 1],
        [0, 4, 5],
        [4, 6, 2],
    ]
    .into_iter()
    .map(|x| x.to_vec())
    .collect::<Vec<_>>();
    tracing::info!("{}", Solution::count_paths(7, roads));
}
