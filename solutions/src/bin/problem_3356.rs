impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let len = nums.len();
        let mut diff_array = vec![0; len + 1];
        let (mut sum, mut queries_idx) = (0, 0);
        for (idx, &num) in nums.iter().enumerate() {
            while num > sum + diff_array[idx] {
                if queries_idx == queries.len() {
                    return -1;
                }
                let left_idx = queries[queries_idx][0] as usize;
                let right_idx = queries[queries_idx][1] as usize;
                let value = queries[queries_idx][2];
                queries_idx += 1;
                if right_idx < idx {
                    continue;
                }
                diff_array[left_idx.max(idx)] += value;
                diff_array[right_idx + 1] -= value;
            }
            sum += diff_array[idx];
        }
        queries_idx as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::min_zero_array(
            vec![2, 0, 2],
            vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
        )
    );
}
