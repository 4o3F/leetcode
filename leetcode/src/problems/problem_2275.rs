impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut count = vec![0; 24];

        for num in &candidates {
            (0..24).for_each(|idx| {
                if (num >> idx) & 1 == 1 {
                    count[idx] += 1;
                }
            });
        }
        count.iter().max().unwrap().clone()
    }
}

struct Solution {}
pub fn run() {
    let arr = vec![16, 17, 71, 62, 12, 24, 14];
    tracing::info!("{}", Solution::largest_combination(arr));
    let arr = vec![8, 8];
    tracing::info!("{}", Solution::largest_combination(arr));
}
