impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut cur = 1;
        k -= 1;
        while k > 0 {
            let steps = Self::count(n as i64, cur as i64);
            if steps <= k as i64 {
                cur += 1;
                k -= steps as i32;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }

    fn count(n: i64, mut x: i64) -> i64 {
        let mut res = 0;
        let mut nx = x + 1;
        while x <= n {
            res += nx.min(n + 1) - x;
            x *= 10;
            nx *= 10;
        }
        res
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::find_kth_number(13, 2));
    tracing::info!("{}", Solution::find_kth_number(1, 1));
}
