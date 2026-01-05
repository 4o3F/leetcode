impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut comb = 1 << (n - 1);
        if k > 3 * comb {
            return "".into();
        }
        k -= 1;
        let mut res = vec![b'a' + (k / comb) as u8];
        while comb > 1 {
            k %= comb;
            comb /= 2;
            let p = res[res.len() - 1];
            res.push(if k < comb {
                b'a' + (p == b'a') as u8
            } else {
                b'c' - (p == b'c') as u8
            });
        }
        String::from_utf8(res).unwrap()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::get_happy_string(1, 3));
}
