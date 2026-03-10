use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = if nums[0] <= nums[nums.len() - 1] {
            0
        } else {
            let mut size = nums.len();
            let mut base = 0usize;

            while size > 1 {
                let half = size / 2;
                let mid = base + half;

                if nums[mid] > nums[nums.len() - 1] {
                    base = mid;
                }

                size -= half;
            }

            (base + 1) % nums.len()
        };
        let mut size = nums.len();
        let mut base = pivot;
        while size > 1 {
            let half = size / 2;
            let mid = (base + half) % nums.len();

            match unsafe { nums.get_unchecked(mid).cmp(&target) } {
                Equal => return mid as i32,
                Greater => {}
                Less => base = mid,
            }

            size -= half;
        }
        if unsafe { nums.get_unchecked(base).cmp(&target) } == Equal {
            base as i32
        } else {
            -1
        }
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::search(vec![1, 3], 0));
}
