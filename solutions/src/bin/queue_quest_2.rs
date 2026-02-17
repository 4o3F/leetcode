use std::collections::VecDeque;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut queue = tickets.into_iter().enumerate().collect::<VecDeque<_>>();
        let mut time = 0;
        while let Some((idx, ticket)) = queue.pop_front() {
            time += 1;
            if ticket == 1 {
                if idx == k as usize {
                    break;
                }
            } else {
                queue.push_back((idx, ticket - 1));
            }
        }
        time
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();

    tracing::info!("{}", Solution::time_required_to_buy(vec![5, 1, 1, 1], 0));
}
