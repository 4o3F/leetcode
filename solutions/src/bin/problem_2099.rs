impl Solution {
    pub fn max_subsequence(mut n: Vec<i32>, k: i32) -> Vec<i32> {
        for i in 0..n.len() {
            n[i] = (n[i] << 11) | i as i32
        }
        n.sort_unstable();
        let l = n.len() - k as usize;
        n[l..].sort_unstable_by_key(|x| x & ((1 << 11) - 1));
        for x in &mut n {
            *x >>= 11
        }
        n[l..].to_vec()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{:?}", Solution::max_subsequence(vec![2, 1, 3, 3], 2))
}
