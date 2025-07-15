impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut mask = 0;
        for op in operations.into_iter().rev() {
            mask = (mask << 1) | op as i64;
        }
        (b'a' + (((k - 1) as i64 & mask).count_ones() % 26) as u8) as char
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::kth_character(5, Vec::from([0, 0, 0])))
}
