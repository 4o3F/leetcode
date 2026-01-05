use std::collections::HashMap;

impl Solution {
    fn dfs(
        memo: &mut HashMap<u64, i32>,
        index: usize,
        s: &Vec<char>,
        char_mask: i32,
        can_change: bool,
        k: i32,
    ) -> i32 {
        if index == s.len() {
            return 0;
        }

        let key =
            ((index as u64) << 27) | ((char_mask as u64) << 1) | (if can_change { 1 } else { 0 });
        if let Some(&val) = memo.get(&key) {
            return val;
        }

        let shift = (s[index] as u8 - b'a') as u32;
        let current_bit = 1i32 << shift;
        let updated_mask = char_mask | current_bit;

        let mut max_partitions: i32;
        if (updated_mask as u32).count_ones() as i32 > k {
            max_partitions = 1 + Self::dfs(memo, index + 1, s, current_bit, can_change, k);
        } else {
            max_partitions = Self::dfs(memo, index + 1, s, updated_mask, can_change, k);
        }

        if can_change {
            for c in 0..26 {
                let new_bit = 1i32 << (c as u32);
                let new_mask = char_mask | new_bit;
                let partitions = if (new_mask as u32).count_ones() as i32 > k {
                    1 + Self::dfs(memo, index + 1, s, new_bit, false, k)
                } else {
                    Self::dfs(memo, index + 1, s, new_mask, false, k)
                };
                if partitions > max_partitions {
                    max_partitions = partitions;
                }
            }
        }

        memo.insert(key, max_partitions);
        max_partitions
    }

    pub fn max_partitions_after_operations(input: String, k: i32) -> i32 {
        let mut sol = HashMap::new();
        let chars: Vec<char> = input.chars().collect();
        1 + Self::dfs(&mut sol, 0, &chars, 0, true, k)
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_partitions_after_operations("accca".to_string(), 2),
    )
}
