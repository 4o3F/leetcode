impl Solution {
    pub fn max_value(mut es: Vec<Vec<i32>>, k: i32) -> i32 {
        es.sort_unstable_by_key(|e| e[1]);
        let mut dp1 = vec![[0, 0]];
        for _ in 0..k {
            let mut dp2 = vec![[0, 0]];
            for e in &es {
                let mut lo = 0;
                let mut hi = dp1.len() - 1;
                while lo <= hi {
                    let m = (lo + hi) / 2;
                    if dp1[m][0] < e[0] {
                        lo = m + 1
                    } else {
                        hi = m - 1
                    }
                }
                if dp1[hi][1] + e[2] > dp2[dp2.len() - 1][1] {
                    dp2.push([e[1], e[2] + dp1[hi][1]])
                }
            }
            dp1 = dp2
        }
        dp1[dp1.len() - 1][1]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2)
    )
}
