impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        spaces.iter().rev().fold(s.clone(), |mut result, &space| {
            result.insert(space as usize, ' ');
            result
        })
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
    );
}
