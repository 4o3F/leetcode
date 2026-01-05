use std::collections::{HashMap, HashSet};

impl Solution {
    fn count_idx_map(map: &HashMap<i32, HashSet<usize>>) -> i64 {
        let mut count = 0i64;
        for value in map.values() {
            let idx_count = value.len() as i64 - 1;
            count += (1 + idx_count) * idx_count / 2;
        }
        count
    }

    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let length = nums.len();
        let k = k as i64;
        let mut good_subarray_count = 0i64;

        let mut index_map = HashMap::<i32, HashSet<usize>>::new();

        let mut right_idx: Option<usize> = None;
        let mut idx_map_count: Option<i64> = None;
        for left_idx in 0..length - 1 {
            if right_idx.is_none() {
                right_idx = Some(left_idx + 1);
            }
            index_map
                .entry(nums[left_idx])
                .or_default()
                .insert(left_idx);

            let right_idx = right_idx.as_mut().unwrap();

            index_map
                .entry(nums[*right_idx])
                .or_default()
                .insert(*right_idx);

            if idx_map_count.is_none() {
                idx_map_count = Some(Self::count_idx_map(&index_map));
            }
            let idx_map_count = idx_map_count.as_mut().unwrap();
            // tracing::info!("left_idx {} idx_map_count {}", left_idx, idx_map_count);

            while *right_idx < length - 1 {
                if *idx_map_count < k {
                    *right_idx += 1;
                    let entry: &mut HashSet<usize> = index_map.entry(nums[*right_idx]).or_default();
                    let current_entry_count = entry.len() as i64;
                    *idx_map_count += current_entry_count;
                    entry.insert(*right_idx);
                    // tracing::info!("{:?} {}", index_map, *idx_map_count);
                } else {
                    break;
                }
            }
            if *idx_map_count >= k {
                // tracing::info!("{} {}", left_idx, right_idx);
                good_subarray_count += (length - *right_idx) as i64;
            }

            let left_entry = index_map.entry(nums[left_idx]).or_default();
            left_entry.remove(&left_idx);
            *idx_map_count -= left_entry.len() as i64;
        }
        good_subarray_count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_good(vec![1, 1, 1, 1, 1], 10));
    tracing::info!("{}", Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2));
}
