impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let c = s.as_bytes();
        let n = c.len();
        let mut p = vec![-1; n + 1];
        for i in 0..n {
            p[i + 1] = if i == 0 || c[i - 1] == b'0' {
                i as i32
            } else {
                p[i]
            };
        }
        let mut result = 0;
        for i in 1..=n {
            let mut z = if c[i - 1] == b'0' { 1 } else { 0 };
            let mut j = i as i32;
            while j > 0 && (z * z) as usize <= n {
                let o = (i as i32 - p[j as usize]) - z;
                if z * z <= o {
                    result += (j - p[j as usize]).min(o - z * z + 1);
                }
                j = p[j as usize];
                z += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::number_of_substrings("00011".to_string()))
}
