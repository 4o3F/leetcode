impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; a.len()];
        let mut results = Vec::new();
        for i in 0..a.len() {
            let mut result = *results.last().unwrap_or(&0);
            freq[a[i] as usize - 1] += 1;
            if freq[a[i] as usize - 1] == 2 {
                result += 1;
            }
            freq[b[i] as usize - 1] += 1;
            if freq[b[i] as usize - 1] == 2 {
                result += 1;
            }
            results.push(result);
        }
        results
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4])
    );
    tracing::info!(
        "{:?}",
        Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2])
    );
}
