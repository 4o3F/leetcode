impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let n = n as usize;
        let mut v = vec![1, 2, 2];
        for i in 2..n {
            if v.len() >= n {
                break;
            }
            let x = if v.last() == Some(&2) { 1 } else { 2 };
            for _ in 0..v[i] {
                v.push(x);
            }
        }
        v[0..n].iter().filter(|x| **x == 1).count() as _
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::magical_string(6));
}
