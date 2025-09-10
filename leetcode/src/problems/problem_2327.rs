use std::collections::VecDeque;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let delay = delay as usize;
        let forget = forget as usize;

        let mut starts = VecDeque::<(usize, i64)>::new();
        let mut ends = VecDeque::<(usize, i64)>::new();
        let mut share: i64 = 0;
        let mut alive: i64 = 1;

        starts.push_back((1 + delay, 1));
        ends.push_back((1 + forget, 1));

        for day in 2..=n {
            while let Some(&(d, cnt)) = starts.front() {
                if d != day {
                    break;
                }
                share = (share + cnt) % MOD;
                starts.pop_front();
            }
            while let Some(&(d, cnt)) = ends.front() {
                if d != day {
                    break;
                }
                share = (share - cnt + MOD) % MOD;
                alive = (alive - cnt + MOD) % MOD;
                ends.pop_front();
            }

            if share > 0 {
                alive = (alive + share) % MOD;
                starts.push_back((day + delay, share));
                ends.push_back((day + forget, share));
            }

            // tracing::info!("day={} share={} alive={}", day, share, alive);
        }

        alive as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::people_aware_of_secret(6, 2, 4));
    tracing::info!("{}", Solution::people_aware_of_secret(790, 68, 538));
}
