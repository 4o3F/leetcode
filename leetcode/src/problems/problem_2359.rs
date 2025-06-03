use std::collections::HashMap;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let nodes = edges.len();
        let graph = edges.iter().enumerate().fold(
            HashMap::<i32, i32>::new(),
            |mut graph, (idx, &target)| {
                if target != -1 {
                    graph.insert(idx as i32, target);
                }
                graph
            },
        );

        tracing::info!("{:?}", graph);
        let distance1 = Self::get_distance(&graph, nodes, node1);
        let distance2 = Self::get_distance(&graph, nodes, node2);
        tracing::info!("{:?}", distance1);
        tracing::info!("{:?}", distance2);

        let (mut min_sum_distance, mut node) = (i32::MAX, None);
        for idx in 0..nodes {
            if distance1[idx].is_some() && distance2[idx].is_some() {
                let distance = distance1[idx].unwrap().max(distance2[idx].unwrap());
                if distance < min_sum_distance {
                    node = Some(idx as i32);
                    min_sum_distance = distance;
                }
            }
        }

        if node.is_some() {
            node.unwrap()
        } else {
            -1
        }
    }

    fn get_distance(graph: &HashMap<i32, i32>, nodes: usize, start_node: i32) -> Vec<Option<i32>> {
        let mut distances: Vec<Option<i32>> = vec![None; nodes];

        distances[start_node as usize] = Some(0);
        let mut current_node = start_node;
        let mut distance = 1;
        while graph.contains_key(&current_node) {
            let target_node = *graph.get(&current_node).unwrap();
            if distances[target_node as usize].is_some() {
                break;
            }
            distances[target_node as usize] = Some(distance);
            distance += 1;
            current_node = target_node;
        }
        distances
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::closest_meeting_node(vec![4, 4, 4, 5, 1, 2, 2], 1, 1)
    );
}
