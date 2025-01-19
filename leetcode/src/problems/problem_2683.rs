impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.iter().fold(0, std::ops::BitXor::bitxor) == 0
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::does_valid_array_exist(vec![1, 1, 0]));
}
