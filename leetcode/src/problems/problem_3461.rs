use itertools::Itertools;

impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        while s.len() > 2 {
            s = String::from_utf8(
                s.bytes()
                    .tuple_windows()
                    .map(|(c1, c2)| (c1 + c2) % 10)
                    .collect::<Vec<u8>>(),
            )
            .unwrap();
        }
        let s = s.bytes().collect::<Vec<u8>>();
        s[0] == s[1]
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::has_same_digits("3902".to_string()))
}
