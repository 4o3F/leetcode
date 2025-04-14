impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        fn cnt(x: String, l: u32, s: &str) -> i64 {
            let mut lo = b'0';
            let hi = lo + l as u8;
            let max: String = x
                .bytes()
                .map(|c| {
                    if c > hi {
                        lo = hi;
                    };
                    c.min(hi).max(lo) as char
                })
                .collect();
            let a =
                i64::from_str_radix(&max[..max.len().saturating_sub(s.len())], l + 1).unwrap_or(0);
            let b =
                i64::from_str_radix(&max[max.len().saturating_sub(s.len())..], l + 1).unwrap_or(0);
            a - (b < i64::from_str_radix(s, l + 1).unwrap()) as i64
        }
        cnt(finish.to_string(), limit as u32, &s) - cnt((start - 1).to_string(), limit as u32, &s)
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::number_of_powerful_int(1, 6000, 4, "124".to_string())
    )
}
