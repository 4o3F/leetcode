use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let (m, n, mut r) = (height_map.len(), height_map[0].len(), 0);
        let mut q = BinaryHeap::new();
        for y in 0..m {
            for x in 0..n {
                if y.min(x) < 1 || y == m - 1 || x == n - 1 {
                    q.push((-height_map[y][x], y, x))
                }
            }
        }
        while let Some((min, y, x)) = q.pop() {
            height_map[y][x] = -1;
            let min = -min;
            for (y1, x1) in [(y, x - 1), (y - 1, x), (y, x + 1), (y + 1, x)] {
                if (0..m).contains(&y1) && (0..n).contains(&x1) && height_map[y1][x1] >= 0 {
                    q.push((-min.max(height_map[y1][x1]), y1, x1));
                    r += 0.max(min - height_map[y1][x1]);
                    height_map[y1][x1] = -1
                }
            }
        }
        r
    }
}
struct Solution;
pub fn run() {
    tracing::info!(
        "{}",
        Solution::trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ])
    );
}
