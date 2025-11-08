use itertools::Itertools;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        nums.windows(k as usize)
            .map(|w| {
                let mut f = [0; 51];
                for &v in w.iter() {
                    f[v as usize] += 1
                }
                (0..51)
                    .map(|v| (-f[v], -(v as i32)))
                    .sorted()
                    .into_iter()
                    .take(x as usize)
                    .map(|(f, v)| v * f)
                    .sum()
            })
            .collect()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)
    )
}
