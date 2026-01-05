impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let mut digits: Vec<i32> = Vec::new();
        let mut num = num;
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            digits.push(digit);
        }
        let max_mapper = digits.iter().rfind(|&digit| *digit != 9).unwrap_or(&9);
        let min_mapper = digits.iter().rfind(|&digit| *digit != 0).unwrap_or(&0);

        // tracing::info!("{:?}", digits);
        let mut max_value = 0;
        for digit in digits.iter().rev() {
            let mut digit = *digit;
            if digit == *max_mapper {
                digit = 9;
            }
            max_value *= 10;
            max_value += digit;
        }
        let mut min_value = 0;
        for digit in digits.iter().rev() {
            let mut digit = *digit;
            if digit == *min_mapper {
                digit = 0;
            }
            min_value *= 10;
            min_value += digit;

            // tracing::info!("{}", min_value);
        }
        max_value - min_value
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_max_difference(99999))
}
