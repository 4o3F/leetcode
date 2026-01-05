impl Solution {
    fn flip(i: i32) -> i32 {
        match i {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        }
    }
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..=nums.len() - 3 {
            if nums[i] == 0 {
                nums[i] = 1;
                nums[i + 1] = Self::flip(nums[i + 1]);
                nums[i + 2] = Self::flip(nums[i + 2]);
                count += 1;
                // tracing::info!("flipped {:?}", nums);
            }
        }
        if nums[nums.len() - 3] == 1 && nums[nums.len() - 2] == 1 && nums[nums.len() - 1] == 1 {
            count
        } else {
            -1
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_operations(vec![0, 1, 1, 1, 0, 0]));
}
