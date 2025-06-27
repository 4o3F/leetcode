impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let (mut x, mut l, s) = (0, 0, s.as_bytes());
        for i in (0..s.len()).rev() {
            if s[i] == b'0' {
                l += 1
            } else if l < 31 {
                let y = x + (1 << l);
                if y <= k {
                    x = y;
                    l += 1
                }
            }
        }
        l
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::longest_subsequence("1001010".to_string(), 5)
    )
}
