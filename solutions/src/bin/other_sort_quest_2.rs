impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::<(usize, usize)>::new();
        stack.push((0, nums.len() - 1));
        while let Some((left, right)) = stack.pop() {
            if left < right {
                // tracing::info!("{} {} {:?}", left, right, nums);

                let (lower_bound, greater_bound) = Self::partition(&mut nums, left, right);

                if lower_bound > left {
                    stack.push((left, lower_bound - 1));
                }
                if greater_bound < right {
                    stack.push((greater_bound + 1, right));
                }
            }
        }
        nums
    }

    // return the lower_bound and greater_bound
    fn partition(nums: &mut [i32], left: usize, right: usize) -> (usize, usize) {
        let pivot_idx = left + (right - left) / 2;
        let pivot = nums[pivot_idx];
        nums.swap(left, pivot_idx);
        let mut lower_bound = left;
        let mut greater_bound = right;
        let mut current_idx = left + 1;
        while current_idx <= greater_bound {
            match nums[current_idx].cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    nums.swap(lower_bound, current_idx);
                    lower_bound += 1;
                    current_idx += 1;
                }
                std::cmp::Ordering::Equal => {
                    current_idx += 1;
                }
                std::cmp::Ordering::Greater => {
                    nums.swap(greater_bound, current_idx);
                    greater_bound -= 1;
                }
            }
        }
        (lower_bound, greater_bound)
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::sort_array(vec![5, 2, 1, 3, 1]));
}
