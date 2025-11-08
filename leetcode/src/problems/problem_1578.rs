impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let (mut ans, mut l) = (0, 0);

        for r in 1..colors.len() {
            if colors[r..=r] != colors[l..=l] {
                l = r;
            } else if needed_time[r] < needed_time[l] {
                ans += needed_time[r];
            } else {
                ans += needed_time[l];
                l = r;
            }
        }

        ans
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
    );
}
