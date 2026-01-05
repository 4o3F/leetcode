use std::collections::VecDeque;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;

        let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
        queue.push_back(rooms[0].clone());
        while let Some(room) = queue.pop_front() {
            for key in room {
                if !visited[key as usize] {
                    visited[key as usize] = true;
                    queue.push_back(rooms[key as usize].clone());
                }
            }
        }

        visited.iter().all(|x| *x)
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
    tracing::info!("{}", Solution::can_visit_all_rooms(rooms));
    let rooms = vec![vec![1], vec![2], vec![3], vec![0]];
    tracing::info!("{}", Solution::can_visit_all_rooms(rooms));
}
