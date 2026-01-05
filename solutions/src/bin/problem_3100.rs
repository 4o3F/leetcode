impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut full_bottles = num_bottles;
        let mut empty_bottles = 0;
        let mut drinked = 0;
        loop {
            // tracing::info!("{} {} {} {}", full_bottles, empty_bottles, num_exchange, drinked);
            drinked += full_bottles;
            empty_bottles += full_bottles;
            full_bottles = 0;
            if empty_bottles < num_exchange {
                break;
            } else {
                while empty_bottles >= num_exchange {
                    empty_bottles -= num_exchange;
                    full_bottles += 1;
                    num_exchange += 1;
                }
            }
        }
        drinked
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_bottles_drunk(13, 6));
    tracing::info!("{}", Solution::max_bottles_drunk(10, 3));
}
