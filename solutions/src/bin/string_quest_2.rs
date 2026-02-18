impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut s = s.chars().filter(|&x| x != '-').collect::<String>();
        s = s.to_uppercase();
        let mut result = Vec::<String>::with_capacity(s.len() / k as usize + 1);
        let mut s_iter = s.chars();

        let first_part = s_iter
            .by_ref()
            .take(s.len() % k as usize)
            .collect::<String>();
        if !first_part.is_empty() {
            result.push(first_part);
        }

        for _ in 0..(s.len() / k as usize) {
            let part = s_iter.by_ref().take(k as usize).collect::<String>();
            result.push(part);
        }

        result.join("-")
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::license_key_formatting("2-4A0r7-4k".to_string(), 4)
    );
}
