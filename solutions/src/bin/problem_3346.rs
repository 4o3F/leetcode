use std::cmp::Ordering;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort_unstable();

        let len = nums.len();
        let mut max_freq = 0_usize;
        let num_operations_usize = num_operations as usize;

        let num_min = nums[0];
        let num_max = nums[len - 1];

        for num in num_min..=num_max {
            let free_cost_left = lower_bound(&nums, &(num));
            let free_cost_right = upper_bound(&nums, &(num));
            let total_free_cost = free_cost_right - free_cost_left;

            let left = lower_bound(&nums, &(num - k));
            let right = upper_bound(&nums, &(num + k));

            let total = right - left;
            let total_with_cost = total - total_free_cost;

            let curr_freq = total_free_cost + total_with_cost.min(num_operations_usize);

            max_freq = max_freq.max(curr_freq);
        }

        max_freq as i32
    }
}

#[inline]
fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    arr.binary_search_by(|x| match x.cmp(target) {
        Ordering::Greater => Ordering::Greater,
        _ => Ordering::Less,
    })
    .unwrap_or_else(|x| x)
}

#[inline]
fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    arr.binary_search_by(|num| match num.cmp(target) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    })
    .unwrap_or_else(|x| x)
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_frequency(vec![1, 4, 5], 1, 2))
}
