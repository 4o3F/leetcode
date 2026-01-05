use std::num::NonZeroI32;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .filter_map(|line| {
                NonZeroI32::new(line.bytes().filter(|&byte| byte == b'1').count() as i32)
            })
            .fold((0, 0), |(res, prev), line| {
                (res + prev * line.get(), line.get())
            })
            .0
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::number_of_beams(
            vec!["011001", "000000", "010100", "001000"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    )
}
