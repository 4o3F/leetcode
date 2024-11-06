impl Solution {
    pub fn rotate_string(original: String, goal: String) -> bool {
        let mut muted = original.clone().as_bytes().to_vec();
        for _ in 0..original.len() {
            // tracing::debug!("{:?}", muted);
            if muted == goal.as_bytes() {
                return true;
            }
            let last = muted.first().unwrap().clone();
            muted.remove(0);
            muted.push(last);
            
        }
        if muted == goal.as_bytes() {
            true
        } else {
            false
        }
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::rotate_string("abcde".to_string(), "abced".to_string()));
}
