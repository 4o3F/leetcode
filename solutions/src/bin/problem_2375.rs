impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut stack = Vec::new();
        let mut result = String::new();
        let mut num = 1;
        for c in pattern.chars().chain(std::iter::once('I')) {
            stack.push(num);
            num += 1;
            if c == 'I' {
                while let Some(top) = stack.pop() {
                    result.push((top as u8 + b'0') as char);
                }
            }
        }
        result
    }
}

struct Solution;
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::smallest_number("IIIDIDDD".to_string()));
}
