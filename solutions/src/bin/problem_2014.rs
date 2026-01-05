impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let (mut q, mut q1, mut res) = (vec![String::from("")], vec![], "".into());
        while !q.is_empty() {
            for sub in &q {
                for c in 'a'..='z' {
                    let next = format!("{}{}", sub.clone(), c);
                    let mut i = 0;
                    let r = next.len() * (k as usize);
                    for c in s.bytes() {
                        if c == next.as_bytes()[i % next.len()] {
                            i += 1;
                            if i == r {
                                break;
                            }
                        }
                    }
                    if i == r {
                        res = next.clone();
                        q1.push(next)
                    }
                }
            }
            (q, q1) = (q1, q);
            q1.clear()
        }
        res
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2)
    );
}
