impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut change_count = 0;
        let mut index = 0;
        while index < s.len() - 1 {
            if s[index] != s[index + 1] {
                change_count += 1;
            }
            // tracing::debug!("{} {} {} {}", change_count, index, s[index], s[index + 1]);
            index += 2;
        }
        change_count
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::min_changes("1001".to_string()));
    tracing::info!("{}", Solution::min_changes("10".to_string()));
    tracing::info!("{}", Solution::min_changes("0000".to_string()));

}
