impl Solution {
    pub fn smallest_subarrays(n: Vec<i32>) -> Vec<i32> {
        let (mut j, mut r) = ([0; 30], vec![0; n.len()]);
        for i in (0..n.len()).rev() {
            for b in 0..30 {
                if n[i] >> b & 1 > 0 {
                    j[b] = i
                }
            }
            r[i] = 1.max(1 + *j.iter().max().unwrap() as i32 - i as i32)
        }
        r
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]))
}
