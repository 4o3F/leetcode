use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut pos = HashMap::<u8, (usize, usize)>::new();
        for (idx, byte) in bytes.iter().enumerate() {
            let entry = pos.entry(*byte).or_insert((idx, idx));
            if entry.1 < idx {
                entry.1 = idx;
            }
        }
        let mut pos = pos.into_values().collect::<Vec<_>>();
        pos.sort_unstable_by_key(|x| x.0);
        // tracing::info!("{:?}", pos);
        let (mut start, mut end) = pos[0];
        let mut merged = Vec::new();
        for pos in pos.iter() {
            if pos.0 <= end {
                end = pos.1.max(end);
            } else {
                merged.push((start, end));
                start = pos.0;
                end = pos.1;
            }
        }
        merged.push((start, end));
        // tracing::info!("{:?}", merged);
        merged
            .into_iter()
            .map(|x| (x.1 - x.0 + 1) as i32)
            .collect::<Vec<_>>()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
    );
    tracing::info!("{:?}", Solution::partition_labels("eccbbbbdec".to_string()));
}
