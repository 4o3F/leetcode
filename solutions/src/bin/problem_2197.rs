use std::collections::VecDeque;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        (i64::from(a) * i64::from(b) / i64::from(Solution::gcd(a, b))) as i32
    }

    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut result = VecDeque::<i32>::new();
        for num in nums {
            let mut num = num;
            while !result.is_empty() && Solution::gcd(*result.back().unwrap(), num) > 1 {
                num = Solution::lcm(num, result.pop_back().unwrap());
            }
            result.push_back(num);
        }
        result.into_iter().collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::replace_non_coprimes(vec![
            61933, 61933, 61933, 61933, 61933, 19844, 41492, 1886, 4, 1886, 3772, 31, 31, 31, 31,
            75289, 75289, 75289, 75289, 75289, 75289, 5, 25, 925, 6845, 925, 5, 925, 925, 5, 185,
            37, 6845, 185, 961, 1457, 1457, 961, 31, 961, 1457, 1457, 31, 961, 961, 961, 31, 31,
            31, 31, 31, 31, 31, 1457, 1457, 1457, 47, 1457, 961, 1457, 31, 961, 961, 961, 1457,
            1457, 31, 31, 961, 961, 31, 1457, 1457, 31, 24151, 33587, 33587, 11, 121, 407, 407,
            407, 29, 29, 29, 29, 29, 17, 17, 17, 17, 17, 7, 49, 7, 43, 943, 529, 23, 529, 23, 943,
            529, 23, 23, 23, 943, 23, 23, 23, 529, 23, 23, 23, 529, 943, 23, 529, 943, 529, 943,
            529, 943, 23, 23, 943, 41, 23, 23, 529, 529
        ])
    )
}
