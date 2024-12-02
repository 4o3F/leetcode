use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut result = HashSet::<i32>::new();
        for num in arr {
            if result.contains(&(num * 2)) || (num % 2 == 0 && result.contains(&(num / 2))) {
                return true;
            }
            result.insert(num);
        }
        false
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::check_if_exist(vec![10, 2, 5, 3]));
    tracing::info!("{}", Solution::check_if_exist(vec![3, 1, 7, 11]));
}
