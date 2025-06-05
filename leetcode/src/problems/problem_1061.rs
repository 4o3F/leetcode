use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let s1 = s1
            .as_bytes()
            .into_iter()
            .map(|x| x - b'a')
            .collect::<Vec<_>>();

        let s2 = s2
            .as_bytes()
            .into_iter()
            .map(|x| x - b'a')
            .collect::<Vec<_>>();

        let mut map = HashMap::<u8, HashSet<u8>>::new();
        for idx in 0..s1.len() {
            {
                let entry = map.entry(s1[idx]).or_default();
                entry.insert(s2[idx]);
            }
            {
                let entry = map.entry(s2[idx]).or_default();
                entry.insert(s1[idx]);
            }
        }
        // tracing::info!("{:?}", map);

        let mut char_map = HashMap::new();

        for idx in 0..26 {
            if char_map.get(&idx).is_none() {
                let (mut visited, mut min) = (HashSet::new(), idx);
                let mut queue = VecDeque::new();
                queue.push_back(idx);
                while !queue.is_empty() {
                    let top = queue.pop_front().unwrap();
                    if !visited.insert(top) {
                        continue;
                    }
                    if top < min {
                        min = top;
                    }
                    if !map.contains_key(&top) {
                        break;
                    }
                    for c in map[&top].iter() {
                        if !visited.contains(c) {
                            queue.push_back(*c);
                        }
                    }
                }
                // tracing::info!("{:?} min {}", visited, min);
                for x in visited.into_iter() {
                    char_map.insert(x, min);
                }
            }
        }
        // tracing::info!("{:?}", char_map);
        base_str
            .chars()
            .into_iter()
            .map(|x| {
                // tracing::info!("{}", x as u8 - b'a');
                if let Some(c) = char_map.get(&(x as u8 - b'a')) {
                    char::from(*c + b'a')
                } else {
                    x
                }
            })
            .collect()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::smallest_equivalent_string(
            "leetcode".to_string(),
            "programs".to_string(),
            "sourcecode".to_string()
        )
    )
}
