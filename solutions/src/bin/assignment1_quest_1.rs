impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = true;
        for idx in (0..digits.len()).rev() {
            if digits[idx] == 9 {
                if carry {
                    digits[idx] = 0;
                }
            } else if carry {
                digits[idx] += 1;
                carry = false;
            }
        }
        if carry {
            digits.insert(0, 1);
        }
        digits
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::plus_one(vec![4, 3, 2, 1]));
    tracing::info!("{:?}", Solution::plus_one(vec![9]));
    tracing::info!("{:?}", Solution::plus_one(vec![9, 9, 9]));
}
