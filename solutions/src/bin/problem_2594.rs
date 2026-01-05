impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let max_rank = *ranks.iter().max().unwrap() as i64;
        let cars = cars as i64;
        let mut left = 1;
        let mut right = max_rank * cars * cars;

        while left < right {
            let mid = left + (right - left) / 2;
            let mut total = 0;
            for rank in ranks.iter() {
                total += (mid / i64::from(*rank)).isqrt();
                if total >= cars {
                    break;
                }
            }

            if total < cars {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::repair_cars(
            vec![9, 7, 10, 4, 4, 8, 10, 2, 7, 9, 8, 1, 8, 3, 1, 9, 1],
            6977
        )
    );
    tracing::info!("{:?}", Solution::repair_cars(vec![5, 1, 8], 6));
}
