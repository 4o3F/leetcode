impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for (index, operator) in expression.char_indices() {
            if operator == '+' || operator == '-' || operator == '*' {
                let expression_left = Self::diff_ways_to_compute(expression[..index].to_string());
                let expression_right =
                    Self::diff_ways_to_compute(expression[index + 1..].to_string());
                for &left in &expression_left {
                    for &right in &expression_right {
                        match operator {
                            '+' => result.push(left + right),
                            '-' => result.push(left - right),
                            '*' => result.push(left * right),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        if result.is_empty() {
            result.push(expression.parse().unwrap());
        }
        result
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::diff_ways_to_compute("2*3-4*5".to_string())
    );
}
