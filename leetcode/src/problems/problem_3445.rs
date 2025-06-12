impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let (k, s, n, mut r) = (k as usize, s.as_bytes(), s.len(), -(s.len() as i32));
        for &a in b"01234" {
            for &b in b"01234" {
                if a == b {
                    continue;
                }
                let (mut fa, mut pa, mut fb, mut pb, mut seen, mut j) =
                    (0, 0, 0, 0, vec![n as i32; 4], 0);
                for (i, &c) in s.iter().enumerate() {
                    fa += (c == a) as i32;
                    fb += (c == b) as i32;
                    while j + k <= i + 1 && fb >= 2 + pb {
                        let key = ((pa % 2) * 2 + (pb % 2)) as usize;
                        seen[key] = seen[key].min(pa - pb);
                        pa += (s[j] == a) as i32;
                        pb += (s[j] == b) as i32;
                        j += 1;
                    }
                    r = r.max(fa - fb - seen[((1 - fa % 2) * 2 + (fb % 2)) as usize]);
                }
            }
        }
        r
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::max_difference("12233".to_string(), 4));
}
