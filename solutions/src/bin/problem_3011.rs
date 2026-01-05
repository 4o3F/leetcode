impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_by(|a, b| {
            if a.count_ones() == b.count_ones() {
                a.cmp(b)
            } else {
                std::cmp::Ordering::Equal
            }
        });
        nums.windows(2).all(|nums| nums[0] <= nums[1])
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let arr = vec![3, 16, 8, 4, 2];
    tracing::info!("{}", Solution::can_sort_array(arr));
}
