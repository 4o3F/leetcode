impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let (mut prefix, mut prefix_count) = (Vec::<i32>::new(), 0);

        for num in nums.windows(2) {
            prefix.push(prefix_count);
            if num[0] % 2 == num[1] % 2 {
                prefix_count += 1;
            }
        }
        prefix.push(prefix_count);
        tracing::debug!("{:?}", prefix);
        queries
            .iter()
            .map(|query| {
                let (left, right) = (query[0] as usize, query[1] as usize);
                let prefix_count = prefix[right] - if left == 0 { 0 } else { prefix[left] };
                tracing::debug!("{} {} {}", left, right, prefix_count);
                prefix_count == 0
            })
            .collect()
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::is_array_special(vec![2, 2], vec![vec![0, 0]])
    );
    tracing::info!(
        "{:?}",
        Solution::is_array_special(vec![5, 1, 5], vec![vec![0, 1]])
    );
    tracing::info!(
        "{:?}",
        Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]])
    );
    tracing::info!(
        "{:?}",
        Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]])
    )
}
