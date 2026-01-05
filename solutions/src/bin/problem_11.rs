impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.iter().enumerate();
        let mut p1 = iter.next();
        let mut p2 = iter.next_back();
        while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
            result = result.max(h1.min(h2) * (j - i) as i32);
            if h1 < h2 {
                p1 = iter.next();
            } else {
                p2 = iter.next_back();
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
}
