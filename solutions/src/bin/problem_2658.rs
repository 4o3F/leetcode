use std::collections::VecDeque;

impl Solution {
    pub fn find_max_fish(mut g: Vec<Vec<i32>>) -> i32 {
        (0..g.len() * g[0].len())
            .map(|i| {
                let mut s = 0;
                let mut q = VecDeque::from([(i / g[0].len(), i % g[0].len())]);
                while let Some((y, x)) = q.pop_front() {
                    if g[y][x] > 0 {
                        s += g[y][x];
                        g[y][x] = 0;
                        if y > 0 {
                            q.push_back((y - 1, x))
                        }
                        if x > 0 {
                            q.push_back((y, x - 1))
                        }
                        if y < g.len() - 1 {
                            q.push_back((y + 1, x))
                        }
                        if x < g[0].len() - 1 {
                            q.push_back((y, x + 1))
                        }
                    }
                }
                s
            })
            .max()
            .unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::find_max_fish(vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0]
        ])
    );
}
