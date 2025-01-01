impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut p = 0;
        values
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let c = n - i as i32 + p;
                p = p.max(i as i32 + n);
                c
            })
            .max()
            .unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6])
    );
}
