impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn contains_zero(mut num: i32) -> bool {
            while num > 0 {
                if num % 10 == 0 {
                    return true;
                }
                num /= 10;
            }
            false
        }

        for i in 1..n {
            let j = n - i;
            if !contains_zero(i) && !contains_zero(j) {
                return vec![i, j];
            }
        }
        vec![]
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::get_no_zero_integers(2));
}
