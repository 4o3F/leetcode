impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let (mut i, mut j, s1, s2) = (0, 0, str1.as_bytes(), str2.as_bytes());
        while i < str2.len() && j < str1.len() {
            if (s2[i] - s1[j] + 26) % 26 <= 1 {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == s2.len()
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::can_make_subsequence("abc".to_string(), "ad".to_string()));
}
