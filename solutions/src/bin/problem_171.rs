use utils::logger::init_logger;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;
        for (idx, c) in column_title.chars().rev().enumerate() {
            let length = (c as i32 - b'A' as i32 + 1) * 26_i32.pow(idx as u32);
            result += length;
        }
        result
    }
}

struct Solution;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::title_to_number("AB".to_string()));
    tracing::info!("{}", Solution::title_to_number("ZY".to_string()));
}
