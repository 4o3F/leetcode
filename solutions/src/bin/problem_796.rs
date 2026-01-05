impl Solution {
    pub fn rotate_string(original: String, goal: String) -> bool {
        let mut muted = original.clone().as_bytes().to_vec();
        for _ in 0..original.len() {
            // tracing::debug!("{:?}", muted);
            if muted == goal.as_bytes() {
                return true;
            }
            let last = *muted.first().unwrap();
            muted.remove(0);
            muted.push(last);
        }
        muted == goal.as_bytes()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::rotate_string("abcde".to_string(), "abced".to_string())
    );
}
