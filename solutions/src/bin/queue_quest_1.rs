use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut sandwiches = sandwiches.into_iter().rev().collect::<Vec<i32>>();
        let mut queue = students
            .into_iter()
            .map(|x| (usize::MAX, x))
            .collect::<VecDeque<_>>();

        while let Some(&(last_seen, prefer)) = queue.front()
            && let Some(&current_sandwich) = sandwiches.last()
        {
            if last_seen == sandwiches.len() {
                break;
            }

            if current_sandwich == prefer {
                sandwiches.pop();
                queue.pop_front();
            } else {
                queue.pop_front();
                queue.push_back((sandwiches.len(), prefer));
            }
        }

        queue.len() as i32
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1])
    )
}
