impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let (m_const, n, m, k, mut ncr, mut den) =
            (1_000_000_007, n as i64, m as i64, k as i64, 1, 1);
        fn pow(a: i64, b: i64, m_const: i64) -> i64 {
            if b == 0 {
                1
            } else {
                (pow((a * a) % m_const, b / 2, m_const) * if b % 2 > 0 { a } else { 1 }) % m_const
            }
        }
        for i in 1..=k {
            ncr = (ncr * (n - i)) % m_const;
            den = (den * i) % m_const
        }
        ncr = (ncr * pow(den, m_const - 2, m_const)) % m_const;
        ((((m * ncr) % m_const) * pow(m - 1, n - k - 1, m_const)) % m_const) as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_good_arrays(3, 2, 1))
}
