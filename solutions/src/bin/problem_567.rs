impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut b1 = s1.as_bytes().to_vec();
        let b2 = s2.as_bytes();
        b1.sort_unstable();
        b2.windows(b1.len()).any(|substr| {
            let mut ordered = substr.to_vec();
            ordered.sort_unstable();
            ordered == b1
        })
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    tracing::info!("{:?}", Solution::check_inclusion(s1, s2));
}
