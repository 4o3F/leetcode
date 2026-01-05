impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        for pair in conflicting_pairs.iter_mut() {
            if pair[1] < pair[0] {
                pair.swap(0, 1);
            }
        }
        conflicting_pairs.sort_by_key(|pair| pair[1]);
        let m = conflicting_pairs.len();
        let mut max1 = 0;
        let mut max2 = 0;
        let mut gain = 0_i64;
        let mut max_gain = 0_i64;
        let mut total_occupied = 0_i64;
        for i in 0..m {
            let start = conflicting_pairs[i][0] as i64;
            let end = conflicting_pairs[i][1] as i64;
            let mut base = n + 1 - end;
            if i < m - 1 {
                let next_end = conflicting_pairs[i + 1][1] as i64;
                base = next_end - end;
            }
            if start > max1 {
                max2 = max1;
                max1 = start;
                gain = 0;
            } else if start > max2 {
                max2 = start;
            }
            gain += (max1 - max2) * base;
            total_occupied += max1 * base;
            max_gain = max_gain.max(gain);
        }
        let total = n * (n + 1) / 2;
        total - total_occupied + max_gain
    }
}
struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_subarrays(4, vec![vec![2, 3], vec![1, 4]])
    )
}
