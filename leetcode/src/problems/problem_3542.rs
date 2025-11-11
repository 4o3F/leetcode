impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let (mut result, mut stack) = (0, vec![0]);
        for x in nums {
            while stack[stack.len() - 1] > x {
                stack.pop();
            }
            if stack[stack.len() - 1] < x {
                result += 1;
                stack.push(x)
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::min_operations(vec![0, 2]))
}
