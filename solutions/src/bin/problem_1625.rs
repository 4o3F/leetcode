impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        }

        let n = s.len();
        let bytes = s.as_bytes();
        let doubled: Vec<u8> = [bytes, bytes].concat();
        let g = gcd(b as usize, n);
        let mut res = bytes.to_vec();

        let add = |t: &mut [u8], start: usize| {
            let original = t[start] - b'0';
            let mut best = 10u8;
            let mut best_times = 0;
            for i in 0..10 {
                let added = (original + i * (a as u8)) % 10;
                if added < best {
                    best = added;
                    best_times = i;
                }
            }
            let delta = (best_times * (a as u8)) % 10;
            for i in (start..n).step_by(2) {
                t[i] = b'0' + ((t[i] - b'0' + delta) % 10);
            }
        };

        for i in (0..n).step_by(g) {
            let mut t = doubled[i..i + n].to_vec();
            add(&mut t, 1);
            if b % 2 != 0 {
                add(&mut t, 0);
            }
            if t < res {
                res = t;
            }
        }

        String::from_utf8(res).unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::find_lex_smallest_string("5525".to_string(), 9, 2)
    )
}
