impl Solution {
    pub fn maximum_length(n: Vec<i32>, k: i32) -> i32 {
        (0..k as usize)
            .map(|v| {
                let k = k as usize;
                let mut len = vec![0; k];
                n.iter()
                    .map(|&x| {
                        let x = x as usize;
                        len[x % k] = len[x % k].max(1 + len[(k + v - x % k) % k]);
                        len[x % k]
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap() as _
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::maximum_length(vec![1, 2, 3, 4, 5], 2))
}
