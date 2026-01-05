use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let k = n / 3;
        let mut left_mins = vec![0i64; n];
        let mut right_maxs = vec![0i64; n];

        let mut left_heap = BinaryHeap::<i64>::new();
        let mut left_sum = 0i64;

        for &num in &nums[..k] {
            let num = i64::from(num);
            left_heap.push(num);
            left_sum += num;
        }
        left_mins[k - 1] = left_sum;

        for i in k..n - k {
            if i64::from(nums[i]) < *left_heap.peek().unwrap() {
                left_sum += i64::from(nums[i]) - left_heap.pop().unwrap();
                left_heap.push(i64::from(nums[i]));
            }
            left_mins[i] = left_sum;
        }

        let mut right_heap = BinaryHeap::new(); // min-heap with Reverse
        let mut right_sum = 0i64;

        for &num in &nums[n - k..] {
            let num = i64::from(num);
            right_heap.push(Reverse(num));
            right_sum += num;
        }
        right_maxs[n - k] = right_sum;

        for i in (k - 1..=n - k - 1).rev() {
            if i64::from(nums[i]) > right_heap.peek().unwrap().0 {
                right_sum += i64::from(nums[i]) - right_heap.pop().unwrap().0;
                right_heap.push(Reverse(i64::from(nums[i])));
            }
            right_maxs[i] = right_sum;
        }

        let mut min_diff = i64::MAX;
        for i in k - 1..n - k {
            min_diff = min_diff.min(left_mins[i] - right_maxs[i + 1]);
        }
        min_diff
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::minimum_difference(vec![1, 3, 5, 2, 7, 9, 4, 6, 8])
    )
}
