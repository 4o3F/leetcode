impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let len = colors.len();
        let last_index = colors.len() + k as usize - 1;
        (0..(colors.len() + k as usize))
            .fold((0, 1, colors[0] == 1), |(ans, cnt, prev), i| {
                if i == last_index || prev == (colors[i % len] == 1) {
                    (ans + 0.max(cnt - k + 1), 1, colors[i % len] == 1)
                } else {
                    (ans, cnt + 1, colors[i % len] == 1)
                }
            })
            .0
    }
}

struct Solution {}
fn main() {
    use utils::prelude::*;
    init_logger();
    let colors = vec![0, 1, 0, 1, 0];
    tracing::info!("{}", Solution::number_of_alternating_groups(colors, 3));
    let colors = vec![0, 1, 0, 0, 1, 0, 1];
    tracing::info!("{}", Solution::number_of_alternating_groups(colors, 6));
    let colors = vec![1, 1, 0, 1];
    tracing::info!("{}", Solution::number_of_alternating_groups(colors, 4));
}
