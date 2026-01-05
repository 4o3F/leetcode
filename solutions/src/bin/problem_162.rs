impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let length = nums.len();

        if length == 1 || nums[0] > nums[1] {
            return 0;
        } else if nums[length - 1] > nums[length - 2] {
            return (length - 1) as i32;
        }

        let (mut left_idx, mut right_idx) = (1usize, length - 2);
        while left_idx <= right_idx {
            let mid = left_idx + (right_idx - left_idx) / 2;
            if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
                return mid as i32;
            }
            if nums[mid] > nums[mid - 1] {
                left_idx = mid + 1;
            } else {
                right_idx = mid - 1;
            }
        }

        -1
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_peak_element(vec![1, 2, 3, 1]))
}
