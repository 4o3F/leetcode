impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        for i in 0..=60 {
            let target = num1 as i64 - i as i64 * num2 as i64;
            if target >= 0 && target.count_ones() as i32 <= i && i as i64 <= target {
                return i;
            }
        }
        -1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::make_the_integer_zero(112577768, -501662198))
}
