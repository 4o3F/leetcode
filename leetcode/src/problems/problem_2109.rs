impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        spaces.iter().rev().fold(s.clone(), |mut result, &space| {
            result.insert(space as usize, ' ');
            return result;
        })
    }
}

struct Solution {}
pub fn run() {
    tracing::info!(
        "{}",
        Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
    );
}
