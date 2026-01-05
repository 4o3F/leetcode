use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut end_map = vec![Vec::new(); nums.len()];
        for query in queries {
            end_map[query[0] as usize].push(query[1]);
        }
        let mut heap = BinaryHeap::<i32>::new();
        let mut end_counts = vec![0; nums.len()];

        let mut removed_count = 0;
        for (idx, num) in nums.iter().enumerate() {
            for ends in &end_map[idx] {
                heap.push(*ends);
            }
            let required = num - removed_count;
            if required > 0 {
                if required > heap.len() as i32 {
                    return -1;
                }
                removed_count += required;
                for _ in 0..required {
                    let end = heap.pop().unwrap();
                    if end < idx as i32 {
                        return -1;
                    }
                    end_counts[end as usize] += 1;
                }
            }
            removed_count -= end_counts[idx];
        }

        heap.len() as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_removal(
            vec![1, 1, 1, 1],
            vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]
        )
    )
}
