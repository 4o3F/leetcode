impl Solution {
    fn is_valid(mut num: i32) -> bool {
        let mut digits = Vec::new();
        while num != 0 {
            digits.push(num % 10);
            num /= 10;
        }
        if digits.len() % 2 == 1 {
            return false;
        }
        let length = digits.len() / 2;
        if digits.iter().take(length).sum::<i32>() == digits.iter().skip(length).sum::<i32>() {
            // tracing::info!("{:?}", digits);
            true
        } else {
            false
        }
    }

    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut count = 0;
        for num in low..=high {
            if Self::is_valid(num) {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::count_symmetric_integers(100, 1782));
}
