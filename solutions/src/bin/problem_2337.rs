impl Solution {
    fn to_array(s: String) -> Vec<(char, usize)> {
        let mut result: Vec<(char, usize)> = Vec::new();
        for (index, c) in s.chars().enumerate() {
            if c == '_' {
                continue;
            }
            result.push((c, index));
        }
        result
    }

    pub fn can_change(start: String, target: String) -> bool {
        let start = Self::to_array(start);
        let target = Self::to_array(target);
        tracing::debug!("start: {:?}", start);
        tracing::debug!("target: {:?}", target);
        if start.len() != target.len() {
            return false;
        }
        for (start, target) in start.iter().zip(target.iter()) {
            if start.0 != target.0 {
                return false;
            }
            match start.0 {
                'L' => {
                    if start.1 < target.1 {
                        return false;
                    }
                }
                'R' => {
                    if start.1 > target.1 {
                        return false;
                    }
                }
                _ => {
                    tracing::debug!("{} {}", start.0, target.0);
                    unreachable!();
                }
            }
        }
        true
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::can_change("_R".to_string(), "R_".to_string())
    );
    tracing::info!(
        "{}",
        Solution::can_change("_LL___R__R_".to_string(), "L___L____RR".to_string())
    );
    tracing::info!(
        "{}",
        Solution::can_change("L__R__R_L".to_string(), "L______RR".to_string())
    );
}
