impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target = nums.len() - k as usize;
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left <= right {
            let pivot_idx = left + (right - left) / 2;
            let (lt, gt) = Self::partition(&mut nums, left, right, pivot_idx);

            if target < lt {
                right = lt - 1;
            } else if target > gt {
                left = gt + 1;
            } else {
                return nums[target];
            }
        }

        unreachable!()
    }

    fn partition(nums: &mut [i32], left: usize, right: usize, pivot_idx: usize) -> (usize, usize) {
        let pivot = nums[pivot_idx];
        nums.swap(pivot_idx, right);

        let mut lt = left;
        let mut i = left;
        let mut gt = right;

        while i <= gt {
            if nums[i] < pivot {
                nums.swap(i, lt);
                lt += 1;
                i += 1;
            } else if nums[i] > pivot {
                nums.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            } else {
                i += 1;
            }
        }

        (lt, gt)
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
}
