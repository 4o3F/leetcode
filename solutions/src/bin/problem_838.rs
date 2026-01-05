impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let (mut left, mut right) = (0, 0);
        let mut dominoes = dominoes.into_bytes();
        while left < dominoes.len() - 1 {
            while right < dominoes.len() - 1 && dominoes[right] == b'.' {
                right += 1;
            }

            match (dominoes[left], dominoes[right]) {
                (b'R', b'L') => {
                    tracing::info!("{} {}", left, right);
                    let len = (right - left).div_ceil(2);
                    dominoes[left..left + len].fill(b'R');
                    dominoes[right - len + 1..=right].fill(b'L');
                }
                (b'R', _) => {
                    dominoes[left..=right].fill(b'R');
                }
                (_, b'L') => {
                    dominoes[left..=right].fill(b'L');
                }
                _ => {}
            }
            left = right;
            right += 1;
        }
        String::from_utf8(dominoes).unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::push_dominoes(".L.R...LR..L..".to_string()));
}
