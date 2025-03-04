impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n >= 3 {
            if n % 3 != 0 {
                n -= 1;
                if n % 3 != 0 {
                    return false;
                }
            } else {
                n /= 3;
            }
        }
        n == 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::check_powers_of_three(11));
}
