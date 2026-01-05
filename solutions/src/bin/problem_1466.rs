// TODO: Redo this problem

use std::collections::VecDeque;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut connections_map = vec![vec![]; n as usize];
        for v in connections {
            let v0 = v[0] as usize;
            let v1 = v[1] as usize;
            connections_map[v0].push((v1, true));
            connections_map[v1].push((v0, false));
        }

        let mut count = 0;

        let mut visited = vec![false; n as usize];
        let mut queue = VecDeque::new();
        queue.push_back((0, false));
        while let Some((index, incom)) = queue.pop_front() {
            if visited[index] {
                continue;
            } else {
                visited[index] = true;
            }

            if incom {
                count += 1;
            }

            for &next_node in connections_map[index].iter() {
                queue.push_back(next_node);
            }
        }
        count
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let n = 6;
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    tracing::info!("{}", Solution::min_reorder(n, connections));
}
