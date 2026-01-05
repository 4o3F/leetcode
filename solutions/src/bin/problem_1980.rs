impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = String::new();
        for (index, num) in nums.iter().enumerate() {
            match num.chars().nth(index).unwrap() {
                '0' => result.push('1'),
                '1' => result.push('0'),
                _ => unreachable!(),
            }
        }
        result
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    let nums = ["111", "011", "001"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    tracing::info!("{}", Solution::find_different_binary_string(nums));
}
