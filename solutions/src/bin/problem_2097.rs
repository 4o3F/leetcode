use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut m, mut f) = (HashMap::new(), HashMap::new());
        for p in &pairs {
            m.entry(p[0]).or_insert_with(Vec::new).push(p[1]);
            *f.entry(p[0]).or_insert(0) += 1;
            *f.entry(p[1]).or_insert(0) -= 1;
        }
        let first = f
            .iter()
            .find(|&(_, &v)| v > 0)
            .map(|(k, _)| *k)
            .unwrap_or_else(|| pairs[0][0]);
        let mut stack = vec![first, -1];
        let mut prev = -1;
        let mut res = (0..pairs.len())
            .map(|_| {
                loop {
                    prev = stack.pop().unwrap();
                    while let Some(sibl) = m.get_mut(stack.last().unwrap()) {
                        let Some(s) = sibl.pop() else { break };
                        stack.push(s)
                    }
                    if prev >= 0 {
                        break;
                    }
                }
                vec![*stack.last().unwrap(), prev]
            })
            .collect::<Vec<_>>();
        res.reverse();
        res
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
    tracing::info!("{:?}", Solution::valid_arrangement(pairs));
}
