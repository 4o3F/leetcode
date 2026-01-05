use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        arr.iter()
            .fold(HashMap::<i32, i32>::new(), |mut freq, &num| {
                let entry = freq.entry(num).or_insert(0);
                *entry += 1;
                freq
            })
            .iter()
            .filter(|(&key, &value)| key == value)
            .map(|x| *x.0)
            .max()
            .unwrap_or(-1)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_lucky(vec![2, 2, 3, 4]))
}
