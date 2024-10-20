impl Solution {
    pub fn find_kth_bit(_: i32, mut k: i32) -> char {
        if k % 2 == 1 { return if k % 4 == 1 { '0' } else { '1' } }
        loop {
            k >>= 1;
            if k % 2 == 1 { return if k % 4 == 1 { '1' } else { '0' } }
        }
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::find_kth_bit(3, 1));
}