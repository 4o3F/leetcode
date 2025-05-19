impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort();
        if nums[0] + nums[1] <= nums[2] {
            String::from("none")
        } else if nums[0] == nums[1] && nums[1] == nums[2] {
            String::from("equilateral")
        } else if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] {
            String::from("isosceles")
        } else {
            String::from("scalene")
        }
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::triangle_type(vec![3, 4, 5]))
}
