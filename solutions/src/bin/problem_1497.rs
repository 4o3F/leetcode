impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let (mut exp, mut cnt) = (vec![0; k as usize], 0);
        for x in &arr {
            let e = ((k + x % k) % k) as usize;
            if exp[e] > 0 {
                cnt += 1;
                exp[e] -= 1;
            } else {
                exp[((k - e as i32) % k) as usize] += 1;
            }
        }
        cnt == arr.len() / 2
    }
}
struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5)
    );
}
