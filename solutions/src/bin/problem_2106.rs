impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut left = 0;
        let mut sum = 0;
        let mut max_fruits = 0;

        for right in 0..fruits.len() {
            sum += fruits[right][1];

            while left <= right && Self::min_steps(fruits[left][0], fruits[right][0], start_pos) > k
            {
                sum -= fruits[left][1];
                left += 1;
            }

            max_fruits = max_fruits.max(sum);
        }

        max_fruits
    }

    fn min_steps(left: i32, right: i32, start: i32) -> i32 {
        let go_left = (start - left).abs() + (right - left);
        let go_right = (start - right).abs() + (right - left);
        go_left.min(go_right)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_total_fruits(
            vec![
                vec![0, 9],
                vec![4, 1],
                vec![5, 7],
                vec![6, 2],
                vec![7, 4],
                vec![10, 9]
            ],
            5,
            4
        )
    )
}
