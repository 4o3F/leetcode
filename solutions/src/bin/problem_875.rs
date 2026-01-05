impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut low = 1;
        let mut high = *piles.iter().max().unwrap() as i64;

        while low < high {
            let mid = low + (high - low) / 2;
            // tracing::info!("{} {} {}", mid, low, high);
            if Self::can_eat_all(&piles, h, mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }

    fn can_eat_all(piles: &Vec<i32>, h: i32, k: i64) -> bool {
        let mut total_hour = 0;
        for &pile in piles {
            total_hour += (pile as i64 + k - 1) / k;
            if total_hour > h as i64 {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
}
