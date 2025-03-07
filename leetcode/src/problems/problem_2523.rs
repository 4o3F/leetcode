use std::i32;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = vec![true; right as usize + 1];
        primes[0] = false;
        primes[1] = false;
        let mut i = 0;
        while i * i <= right {
            if primes[i as usize] {
                let mut j = i * i;
                while j <= right {
                    primes[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let primes = (left..=right)
            .filter(|x| primes[*x as usize])
            .collect::<Vec<_>>();

        if primes.len() < 2 {
            return vec![-1, -1];
        }

        let mut result = vec![primes[0], primes[1]];
        for win in primes.windows(2) {
            if win[1] - win[0] < result[1] - result[0] {
                result = win.to_vec();
            }
        }

        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::closest_primes(10, 19));
    tracing::info!("{:?}", Solution::closest_primes(4, 6));
}
