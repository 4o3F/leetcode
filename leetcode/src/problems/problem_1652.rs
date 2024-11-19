impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        (0..code.len() as i32)
            .map(|i| {
                (i.min(i + k)..=i.max(i + k))
                    .map(|j| code[(j as usize + code.len()) % code.len()])
                    .sum::<i32>()
                    - code[i as usize]
            })
            .collect()
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{:?}", Solution::decrypt(vec![5, 7, 1, 4], 3));
}
