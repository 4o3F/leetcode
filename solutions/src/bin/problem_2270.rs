impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let (mut right_sum, mut left_sum, length) = (
            nums.iter().map(i32::clone).map(i64::from).sum::<i64>(),
            0i64,
            nums.len(),
        );
        nums.into_iter()
            .take(length - 1)
            .fold(0, |mut result, num| {
                left_sum += i64::from(num);
                right_sum -= i64::from(num);
                if left_sum >= right_sum {
                    result += 1;
                }
                result
            })
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::ways_to_split_array(vec![10, 4, -8, 7]));
    tracing::info!("{}", Solution::ways_to_split_array(vec![2, 3, 1, 0]));
}
