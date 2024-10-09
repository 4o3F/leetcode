impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let (mut c, mut res) = (0, 0);
        for b in s.bytes() {
            if b == b'[' {
                c += 1;
            } else if c == 0 {
                res += 1;
                c = 1;
            } else {
                c -= 1;
            }
        }
        res
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::min_swaps("][][".to_string()));
}
