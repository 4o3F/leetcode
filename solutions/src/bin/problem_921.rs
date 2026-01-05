impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut balance, mut count) = (0, 0);
        for c in s.chars() {
            if c == '(' {
                balance += 1;
            } else if balance > 0 {
                balance -= 1;
            } else {
                count += 1;
            }
        }
        count + balance
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_add_to_make_valid("())".to_string()));
}
