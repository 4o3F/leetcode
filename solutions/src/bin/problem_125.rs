use utils::logger::init_logger;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let (mut left_ptr, mut right_ptr) = (0, s.len().max(1) - 1);
        let mut result = true;
        while left_ptr != right_ptr && left_ptr < right_ptr {
            if s[left_ptr] != s[right_ptr] {
                result = false;
                break;
            }
            left_ptr += 1;
            right_ptr -= 1;
        }

        result
    }
}

struct Solution;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    tracing::info!("{}", Solution::is_palindrome("race a car".to_string()));
    tracing::info!("{}", Solution::is_palindrome(" ".to_string()));
    tracing::info!("{}", Solution::is_palindrome("0P".to_string()));
}
