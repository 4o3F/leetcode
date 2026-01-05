impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MODULO: i32 = 1_000_000_007;
        let mut freq = [0i32; 26];
        let s = s.into_bytes();
        for c in s {
            freq[(c - b'a') as usize] += 1;
        }
        for _ in 0..t {
            let added_ab = freq[25];
            for i in (1..26).rev() {
                freq[i] = freq[i - 1] % MODULO;
            }
            freq[0] = added_ab % MODULO;
            freq[1] += added_ab % MODULO;
            // tracing::info!("{:?}", freq);
        }
        freq.iter()
            .fold(0i64, |sum, num| (sum + i64::from(*num)) % i64::from(MODULO)) as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::length_after_transformations("iatckelagcytuuxbgiajuoni".to_string(), 6626)
    );
}
