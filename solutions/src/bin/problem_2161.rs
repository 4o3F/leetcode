impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut lower = vec![];
        let mut upper = vec![];
        let mut equal_count = 0;
        for num in nums {
            match num.cmp(&pivot) {
                std::cmp::Ordering::Less => lower.push(num),
                std::cmp::Ordering::Equal => equal_count += 1,
                std::cmp::Ordering::Greater => upper.push(num),
            }
        }
        lower.extend(std::iter::repeat_n(pivot, equal_count));
        lower.extend(upper);
        lower
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
    );
}
