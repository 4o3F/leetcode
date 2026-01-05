impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        // tracing::info!("{:?}", points);
        let n = points.len();
        let mut result = 0;
        for i in 0..n {
            let y_top = points[i][1];
            let mut y_bottom = i32::MIN;
            for j in (i + 1)..n {
                let y = points[j][1];
                if y_bottom < y && y <= y_top {
                    result += 1;
                    y_bottom = y;
                    if y_bottom == y_top {
                        break;
                    }
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::number_of_pairs(vec![vec![0, 0], vec![0, 3]])
    )
}
