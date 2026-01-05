impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];
        for num in nums.iter() {
            counts[*num as usize] += 1;
        }
        let mut current = 0;
        for num in nums {
            while current < counts.len() && counts[current] == 0 {
                current += 1;
            }
            *num = current as i32;
            counts[current] -= 1;
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut nums = vec![2];
    Solution::sort_colors(&mut nums);
    tracing::info!("{:?}", nums);
}
