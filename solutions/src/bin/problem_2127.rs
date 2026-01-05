use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(fav: Vec<i32>) -> i32 {
        let mut deg = vec![0; fav.len()];
        let mut path = deg.clone();
        for i in 0..fav.len() {
            deg[fav[i] as usize] += 1
        }
        let mut q = VecDeque::from_iter((0..fav.len()).filter(|&i| deg[i] == 0));
        while let Some(i) = q.pop_front() {
            let j = fav[i] as usize;
            path[j] = path[i] + 1;
            deg[j] -= 1;
            if deg[j] == 0 {
                q.push_back(j)
            }
        }
        let (mut path_sum, mut cycle_max) = (0, 0);
        for i in 0..fav.len() {
            let (mut cycle, mut j) = (0, i);
            while deg[j] > 0 {
                deg[j] = 0;
                j = fav[j] as usize;
                cycle += 1
            }
            if cycle == 2 {
                path_sum += 2 + path[i] + path[fav[i] as usize]
            } else {
                cycle_max = cycle_max.max(cycle)
            }
        }
        path_sum.max(cycle_max)
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_invitations(vec![2, 2, 1, 2]));
}
