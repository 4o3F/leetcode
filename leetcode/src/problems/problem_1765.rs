use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        for y in 0..is_water.len() {
            for x in 0..is_water[0].len() {
                if is_water[y][x] > 0 {
                    is_water[y][x] = 0;
                    q.push_back((y, x))
                } else {
                    is_water[y][x] = -1
                }
            }
        }
        while let Some((y, x)) = q.pop_front() {
            for (y1, x1) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if y1 < is_water.len() && x1 < is_water[0].len() {
                    if is_water[y1][x1] >= 0 {
                        continue;
                    }
                    is_water[y1][x1] = 1 + is_water[y][x];
                    q.push_back((y1, x1))
                }
            }
        }
        is_water
    }
}

struct Solution;
pub fn run() {
    tracing::info!("{:?}", Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]));
}
