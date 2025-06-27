impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut cs = s.chars().collect::<Vec<_>>();
        if cs.len() % k as usize != 0 {
            for _ in 0..k - (cs.len() as i32 % k) {
                cs.push(fill);
            }
        }
        cs.chunks(k as usize)
            .map(|cs| cs.iter().copied().collect::<String>())
            .collect::<Vec<_>>()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::divide_string("abcdefghi".to_string(), 3, 'x')
    )
}
