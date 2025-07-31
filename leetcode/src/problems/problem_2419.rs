impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut length = 0;

        let mut current_length = 0;

        for num in nums {
            match num.cmp(&max) {
                std::cmp::Ordering::Greater => {
                    max = num;
                    length = 1;
                    current_length = 1;
                }
                std::cmp::Ordering::Equal => {
                    current_length += 1;
                    length = length.max(current_length)
                }
                std::cmp::Ordering::Less => current_length = 0,
            }
        }

        length
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]))
}
