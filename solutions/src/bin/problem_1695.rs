use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut idx_map, mut start_idx, mut sum, mut max) =
            (HashMap::<i32, Option<usize>>::new(), 0, 0, -1);
        for (current_idx, num) in nums.iter().enumerate() {
            let entry = idx_map.entry(*num).or_insert(None);
            match entry {
                Some(last_occur_idx) => {
                    while start_idx <= *last_occur_idx {
                        sum -= nums[start_idx];
                        start_idx += 1;
                    }
                    *entry = Some(current_idx);
                    sum += num;
                }
                None => {
                    *entry = Some(current_idx);
                    sum += num;
                }
            }
            // tracing::info!("{} {} {}", start_idx, current_idx, sum);
            // tracing::info!("{:?}", idx_map);
            max = max.max(sum);
        }
        max
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let result = Solution::maximum_unique_subarray(vec![
        1, 7, 10, 10, 9, 10, 2, 3, 2, 1, 6, 2, 5, 7, 9, 8, 4, 9, 6, 8,
    ]);
    tracing::info!("{}", result)
}
