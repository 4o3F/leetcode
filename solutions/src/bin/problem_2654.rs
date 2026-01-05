impl Solution {
    pub fn min_operations(n: Vec<i32>) -> i32 {
        let o = n.iter().filter(|&&x| x == 1).count() as i32;
        if o > 0 {
            n.len() as i32 - o
        } else {
            fn g(mut a: i32, b: &i32) -> i32 {
                let mut b = *b;
                while b > 0 {
                    let t = b;
                    b = a % b;
                    a = t;
                }
                a
            }
            if n.iter().fold(0, g) > 1 {
                -1
            } else {
                n.len() as i32 - 2
                    + (2..=n.len())
                        .find(|&l| n.windows(l).any(|w| w.iter().fold(0, g) == 1))
                        .unwrap() as i32
            }
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::min_operations(vec![2, 6, 3, 4]))
}
