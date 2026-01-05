impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let (n_odd, n_even) = (1..=n).fold((0, 0), |(n_odd, n_even), num| {
            if num % 2 == 0 {
                (n_odd, n_even + 1)
            } else {
                (n_odd + 1, n_even)
            }
        });

        let (m_odd, m_even) = (1..=m).fold((0, 0), |(m_odd, m_even), num| {
            if num % 2 == 0 {
                (m_odd, m_even + 1)
            } else {
                (m_odd + 1, m_even)
            }
        });

        let (n_odd, n_even, m_odd, m_even) =
            (n_odd as i64, n_even as i64, m_odd as i64, m_even as i64);

        n_odd * m_even + n_even * m_odd
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::flower_game(3, 2))
}
