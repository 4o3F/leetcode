impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut transformed_nums = nums
            .iter()
            .rev()
            .scan(-1i32, |state, &num| {
                if num > *state {
                    *state = num;
                    Some(num)
                } else {
                    Some(*state)
                }
            })
            .collect::<Vec<_>>();
        transformed_nums.reverse();
        // tracing::info!("{:?}", transformed_nums);
        (0..nums.len()).fold(-1, |result, idx| {
            if transformed_nums[idx] - nums[idx] > 0 {
                result.max(transformed_nums[idx] - nums[idx])
            } else {
                result
            }
        })
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum_difference(vec![1, 5, 2, 10]));
}
