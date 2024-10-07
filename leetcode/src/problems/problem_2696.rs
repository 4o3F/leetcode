impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut s = s;
        while s.contains("AB") || s.contains("CD") {
            s = s.replace("AB", "").replace("CD", "");
        }
        s.len() as i32
    }
}
struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::min_length("ABFCACDB".to_string()));
}
