impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let chars = s.chars().collect::<Vec<char>>();

        let mut x = 0;
        chars
            .into_iter()
            .map(|c| {
                if c == '1' {
                    x += 1;
                    0
                } else {
                    x
                }
            })
            .sum()
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::minimum_steps("101".into()));
    tracing::info!("{:?}", Solution::minimum_steps("100".into()));
    tracing::info!("{:?}", Solution::minimum_steps("0111".into()));
}
