impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut s = s;
        s.push('!');
        let s = s.as_bytes();
        let mut result = String::new();
        let mut left = 0usize;
        let mut right = 1usize;
        while right < s.len() {
            while right < s.len() && s[left] == s[right] {
                right += 1;
            }
            if right - left > 2 {
                result.push_str(char::from(s[left]).to_string().repeat(2).as_str());
            } else {
                result.push_str(
                    char::from(s[left])
                        .to_string()
                        .repeat(right - left)
                        .as_str(),
                );
            }
            // tracing::debug!("{} {}", left, right);
            left = right;
            right += 1;
        }
        result
    }
}

// aabbbaaa

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let s = "aab".to_string();
    let result = Solution::make_fancy_string(s);
    tracing::info!("{}", result);
}
