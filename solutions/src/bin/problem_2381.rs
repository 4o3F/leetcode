impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        shifts
            .into_iter()
            .fold(vec![0_i64; s.len()], |mut sweep, shift| {
                let (start, end, dir) = (
                    shift[0] as usize,
                    shift[1] as usize,
                    if shift[2] == 1 { 1 } else { -1 },
                );
                sweep[start] += dir;
                if end < s.len() - 1 {
                    sweep[end + 1] -= dir;
                }
                sweep
            })
            .into_iter()
            .scan(0_i64, |sum, n| {
                *sum += n;
                Some(*sum)
            })
            .enumerate()
            .map(|(i, shift)| {
                let c: u8 = s.as_bytes()[i] - b'a';
                let shift = (shift % 26) as i32;
                let shift = if shift < 0 { shift + 26 } else { shift };
                ((c + shift as u8) % 26 + b'a') as char
            })
            .collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::shifting_letters(
            "abc".to_string(),
            vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
        )
    )
}
