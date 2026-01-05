impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut mask = 0;
        for op in operations.into_iter().rev() {
            mask = (mask << 1) | op as i64;
        }
        (b'a' + (((k - 1) & mask).count_ones() % 26) as u8) as char
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::kth_character(5, Vec::from([0, 0, 0])))
}
