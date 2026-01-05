use std::collections::HashMap;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_map = HashMap::<i32, i32>::new();
        for digit in digits {
            let entry = digits_map.entry(digit).or_insert(0);
            *entry += 1;
        }

        let mut results = Vec::<i32>::new();
        'outer: for result in (100..=999).step_by(2) {
            let mut num = result;
            let mut count = [0; 10];
            while num != 0 {
                let last_digit = num % 10;
                count[last_digit as usize] += 1;
                if !digits_map.contains_key(&last_digit)
                    || digits_map.get(&last_digit).unwrap() < &count[last_digit as usize]
                {
                    continue 'outer;
                }
                num /= 10;
            }
            results.push(result);
        }
        results
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::find_even_numbers(vec![2, 2, 8, 8, 2]))
}
