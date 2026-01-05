impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (a, b) = (1..=n).fold((0, 0), |(not_divisible, divisible), num| {
            if num % m == 0 {
                (not_divisible, divisible + num)
            } else {
                (not_divisible + num, divisible)
            }
        });
        a - b
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::difference_of_sums(10, 3));
}
