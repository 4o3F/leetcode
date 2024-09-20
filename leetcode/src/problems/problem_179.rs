struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.clone();
        nums.sort_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
        let mut result: String = nums.into_iter().map(|n| n.to_string()).collect();


        if result.chars().next().unwrap() == '0'{
            result.truncate(1);
        }

        result
    }
}

pub fn run() {
    let nums = vec![3, 30, 34, 5, 9];
    tracing::info!("{:?}", Solution::largest_number(nums));
}
