use std::collections::VecDeque;

impl Solution {
    pub fn max_task_assign(mut t: Vec<i32>, mut ws: Vec<i32>, pills: i32, s: i32) -> i32 {
        t.sort_unstable();
        ws.sort_unstable();
        let (mut lo, mut hi, mut r) = (0, t.len() as i32 - 1, -1);
        while lo <= hi {
            let m = (lo + hi) as usize / 2;
            let (mut j, mut p, mut good, mut q) = (ws.len() - 1, pills, true, VecDeque::new());
            for i in (0..=m).rev() {
                while j < ws.len() && j >= ws.len() - m - 1 && ws[j] + s >= t[i] {
                    q.push_back(ws[j]);
                    j -= 1
                }
                if q.len() < 1 {
                    good = false;
                    break;
                }
                if *q.front().unwrap() >= t[i] {
                    q.pop_front();
                } else if p < 1 {
                    good = false;
                    break;
                } else {
                    q.pop_back();
                    p -= 1
                }
            }
            if good {
                r = r.max(m as i32);
                lo = m as i32 + 1
            } else {
                hi = m as i32 - 1
            }
        }
        r + 1
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1)
    )
}
