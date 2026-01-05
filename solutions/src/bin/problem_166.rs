use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut res = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            res.push('-');
        }

        let dividend = (numerator as i64).abs();
        let divisor = (denominator as i64).abs();

        res.push_str(&(dividend / divisor).to_string());
        let mut remainder = dividend % divisor;
        if remainder == 0 {
            return res;
        }

        res.push('.');
        let mut seen: HashMap<i64, usize> = HashMap::new();

        while remainder != 0 {
            if let Some(&idx) = seen.get(&remainder) {
                res.insert(idx, '(');
                res.push(')');
                break;
            }
            seen.insert(remainder, res.len());
            remainder *= 10;
            res.push_str(&(remainder / divisor).to_string());
            remainder %= divisor;
        }

        res
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::fraction_to_decimal(1, 2))
}
