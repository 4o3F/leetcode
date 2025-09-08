impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans: i64 = 0;
        for q in queries.iter() {
            let l = q[0] as i64;
            let r = q[1] as i64;
            let mut s: i64 = 0;
            let mut d_max: i64 = 0;
            for k in 1..=31 {
                let low = 1i64 << (k - 1);
                let high = (1i64 << k) - 1;
                if low > r {
                    break;
                }
                let a = l.max(low);
                let b = r.min(high);
                if a > b {
                    continue;
                }
                let cnt = b - a + 1;
                let steps_for_k = ((k + 1) / 2) as i64;
                s += cnt * steps_for_k;
                d_max = d_max.max(steps_for_k);
            }
            let ops = d_max.max((s + 1) / 2);
            ans += ops;
        }
        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::min_operations(vec![vec![1, 2], vec![2, 4]]))
}
