use std::collections::{BinaryHeap, HashSet};

impl Solution {
    fn prime_score(num: i32) -> i32 {
        let mut num = num;
        let mut factor = HashSet::new();

        while num % 2 == 0 {
            num /= 2;
            factor.insert(2);
        }

        let mut i = 3;

        while i * i <= num {
            while num % i == 0 {
                factor.insert(i);
                num /= i;
            }

            i += 2;
        }

        if num > 1 {
            factor.insert(num);
        }

        factor.len() as i32
    }

    fn generate_left(prim_scores: &[i32]) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = vec![-1; prim_scores.len()];
        for (idx, prim_score) in prim_scores.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if prim_score <= &prim_scores[top] {
                    break;
                }
                stack.pop();
            }

            if let Some(&top) = stack.last() {
                result[idx] = top as i32;
            }

            stack.push(idx);
        }
        result
    }

    fn generate_right(prim_scores: &[i32]) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = vec![prim_scores.len() as i32; prim_scores.len()];
        for (idx, prim_score) in prim_scores.iter().enumerate().rev() {
            while let Some(&top) = stack.last() {
                if prim_score < &prim_scores[top] {
                    break;
                }
                stack.pop();
            }

            if let Some(&top) = stack.last() {
                result[idx] = top as i32;
            }

            stack.push(idx);
        }
        result
    }

    fn binpow(x: i64, mut n: i64, p: i64) -> i64 {
        let mut result = 1;
        let mut base = x % p;
        while n > 0 {
            if n & 1 == 1 {
                result = (result * base) % p;
            }
            base = base * base % p;
            n /= 2;
        }
        result
    }

    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let prim_scores = nums
            .iter()
            .map(|x| Self::prime_score(*x))
            .collect::<Vec<_>>();

        // tracing::info!("{:?}", prim_scores);

        let left = Self::generate_left(&prim_scores);
        let right = Self::generate_right(&prim_scores);

        // tracing::info!("{:?}", left);
        // tracing::info!("{:?}", right);

        let mut pq = BinaryHeap::<(i32, i64)>::new();

        for i in 0..nums.len() {
            let i_32 = i as i32;
            if i_32 > left[i] && i_32 < right[i] {
                let range = ((i_32 - left[i]) as i64) * ((right[i] - i_32) as i64);
                pq.push((nums[i], range));
                // tracing::info!("ID {} RANGE {}", i, range);
            }
        }

        // tracing::info!("{:?}", pq);

        let mut result = 1i64;
        let mut k = k as i64;
        while k > 0 {
            let (num, range) = pq.pop().unwrap();
            let pow = range.min(k);
            result *= Self::binpow(num as i64, pow, 1_000_000_007);
            result %= 1_000_000_007;
            // for _ in 0.. {
            //     // tracing::info!("result {} num {}", result, num);
            //     result *= num as i64;
            //     result %= 1_000_000_007;
            // }
            k -= pow;
        }

        result as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_score(vec![8, 3, 9, 3, 8], 2));
    // tracing::info!(
    // "{}",
    // Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3)
    // );
    // tracing::info!(
    //     "{}",
    //     Solution::maximum_score(vec![3289, 2832, 14858, 22011], 6)
    // )
}
