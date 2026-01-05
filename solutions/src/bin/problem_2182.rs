impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut f = [0; 26];
        for b in s.bytes() {
            f[(b - b'a') as usize] += 1
        }
        let (mut cnt, mut i, mut j, mut r) = (0, 25, 25, vec![]);
        loop {
            if f[i] == 0 {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                    continue;
                }
            }
            if r.last().is_none_or(|&l| l == b'a' + i as u8) {
                cnt += 1
            } else {
                cnt = 1
            }
            if cnt > repeat_limit {
                if i == 0 {
                    break;
                } else {
                    j = j.min(i - 1)
                }
                loop {
                    if j == 0 || f[j] > 0 {
                        break;
                    } else {
                        j -= 1
                    }
                }
                if f[j] > 0 {
                    r.push(b'a' + j as u8);
                    f[j] -= 1
                } else {
                    break;
                }
            } else {
                r.push(b'a' + i as u8);
                f[i] -= 1
            }
        }
        String::from_utf8(r).unwrap()
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::repeat_limited_string("cczazcc".to_string(), 3)
    );
}
