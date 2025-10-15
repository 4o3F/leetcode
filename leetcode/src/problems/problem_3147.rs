impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        (0..k)
            .map(|i| {
                energy[i..]
                    .iter()
                    .step_by(k)
                    .rev()
                    .fold((0, i32::MIN), |(sum, max), &v| {
                        let sum = sum + v;
                        (sum, max.max(sum))
                    })
                    .1
            })
            .max()
            .unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3))
}
