use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let graph = equations.into_iter().zip(values).fold(
            HashMap::<String, HashMap<String, f64>>::new(),
            |mut graph, (equation, value)| {
                let entry = graph.entry(equation[0].clone()).or_default();
                entry.insert(equation[1].clone(), value);

                let entry = graph.entry(equation[1].clone()).or_default();
                entry.insert(equation[0].clone(), 1.0 / value);

                graph
            },
        );

        let result = queries
            .into_iter()
            .map(|query| {
                let mut visited = HashSet::new();
                let entry = graph.get(&query[0]);
                if entry.is_none() {
                    return -1.0;
                }

                let mut stack = VecDeque::new();
                stack.push_back((query[0].clone(), query[1].clone(), 1.0));
                while !stack.is_empty() {
                    let current = stack.pop_back();
                    if current.is_none() {
                        break;
                    }

                    let (from, to, value) = current.unwrap();
                    visited.insert(from.clone());
                    if from == to {
                        return value;
                    }

                    let entry = graph.get(&from);

                    if entry.is_none() {
                        break;
                    }

                    let entry = entry.unwrap();
                    for path in entry.keys() {
                        if visited.contains(path) {
                            continue;
                        }
                        stack.push_back((
                            path.clone(),
                            to.clone(),
                            value * entry.get(path).unwrap(),
                        ));
                    }
                }
                -1.0
            })
            .collect::<Vec<f64>>();
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::calc_equation(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()]
            ],
            vec![2.0, 3.0],
            [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
                .into_iter()
                .map(|x| x.into_iter().map(|x| x.to_string()).collect())
                .collect()
        )
    )
}
