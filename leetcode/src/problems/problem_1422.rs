impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut left_0, mut right_1) = (
            s.chars().take(1).filter(|&c| c == '0').count(),
            s.chars().skip(1).filter(|&c| c == '1').count(),
        );
        // tracing::debug!("left_0: {} right_1: {}", left_0, right_1);
        let mut max_score = left_0 + right_1;
        for c in s.chars().skip(1).take(s.len() - 2) {
            if c == '0' {
                left_0 += 1;
            } else {
                right_1 -= 1;
            }
            max_score = max_score.max(left_0 + right_1);
        }
        max_score as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_score("00".to_string()));
    tracing::info!("{}", Solution::max_score("011101".to_string()));
    tracing::info!("{}", Solution::max_score("00111".to_string()));
    tracing::info!("{}", Solution::max_score("1111".to_string()));
}
