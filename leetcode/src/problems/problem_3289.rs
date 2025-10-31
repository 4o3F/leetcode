use std::collections::HashSet;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .fold(
                (HashSet::<i32>::new(), Vec::<i32>::new()),
                |(mut appeared, mut result), num| {
                    if appeared.contains(num) {
                        result.push(*num);
                    } else {
                        appeared.insert(*num);
                    }
                    (appeared, result)
                },
            )
            .1
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{:?}", Solution::get_sneaky_numbers(vec![0, 1, 1, 0]))
}
