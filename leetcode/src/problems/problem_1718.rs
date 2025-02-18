impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let (mut r, mut u) = (vec![0; n as usize * 2 - 1], vec![false; n as usize + 1]);
        fn dfs(i: usize, r: &mut Vec<i32>, u: &mut Vec<bool>) -> bool {
            if i == r.len() {
                return true;
            };
            if r[i] > 0 {
                return dfs(i + 1, r, u);
            }
            for x in (1..u.len()).rev() {
                if !u[x] && (x < 2 || i + x < r.len() && r[i + x] < 1) {
                    u[x] = true;
                    r[i] = x as i32;
                    if x > 1 {
                        r[i + x] = x as i32
                    };
                    if dfs(i + 1, r, u) {
                        return true;
                    };
                    u[x] = false;
                    r[i] = 0;
                    if x > 1 {
                        r[i + x] = 0
                    }
                }
            }
            false
        }
        dfs(0, &mut r, &mut u);
        r
    }
}
struct Solution;
pub fn run() {
    tracing::info!("{:?}", Solution::construct_distanced_sequence(3));
}
