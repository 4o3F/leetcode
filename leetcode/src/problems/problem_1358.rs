impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        s.chars()
            .enumerate()
            .fold((-1, -1, -1, 0), |(a, b, c, m), (i, ch)| {
                let i = i as i32;
                let result = match ch {
                    'a' => (i, b, c, m + i.min(b).min(c) + 1),
                    'b' => (a, i, c, m + i.min(a).min(c) + 1),
                    _ => (a, b, i, m + i.min(a).min(b) + 1),
                };
                // tracing::info!("{:?}", result);
                return result;
            })
            .3
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::number_of_substrings("abcabc".to_string()));
}
