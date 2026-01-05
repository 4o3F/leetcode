impl Solution {
    pub fn max_diff(n: i32) -> i32 {
        let (s, mut a, mut b) = (n.to_string(), n, n);
        for c in s.chars() {
            for d in '0'..='9' {
                let x: String = s.chars().map(|x| if x == c { d } else { x }).collect();
                if x == "0" || x.starts_with("0") {
                    continue;
                }
                let x = x.parse().unwrap();
                a = a.max(x);
                b = b.min(x)
            }
        }
        a - b
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::max_diff(555))
}
