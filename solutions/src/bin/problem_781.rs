use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut hashed_answers = HashMap::<i32, i32>::new();
        for answer in answers {
            let entry = hashed_answers.entry(answer).or_insert(0);
            *entry += 1;
        }

        let mut rabbit_count = 0;
        for (key, value) in hashed_answers {
            let mut value = value;
            while value > key + 1 {
                value -= key + 1;
                rabbit_count += key + 1;
            }
            rabbit_count += key + 1;
            // tracing::info!("key {} color_count {}", key, rabbit_count);
        }

        rabbit_count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::num_rabbits(vec![0, 0, 1, 1, 1]));
    tracing::info!("{}", Solution::num_rabbits(vec![1, 1, 2]));
    tracing::info!("{}", Solution::num_rabbits(vec![10, 10, 10]));
}
