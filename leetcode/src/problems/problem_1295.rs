impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|num| num.to_string().len() % 2 == 0)
            .count() as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::find_numbers(vec![12, 345, 2, 6, 7896]));
}
