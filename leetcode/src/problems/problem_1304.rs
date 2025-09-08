impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n == 2 {
            return Vec::from([-1, 1]);
        }
        let mut result = Vec::new();
        for i in 0i32..(n - 1) {
            result.push(i);
        }
        result.push(-1 * result.iter().sum::<i32>());
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::sum_zero(5))
}
