impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.windows(k as usize)
            .into_iter()
            .map(|sub| {
                if sub
                    .windows(2)
                    .all(|sub| sub[0] < sub[1] && sub[0] + 1 == sub[1])
                {
                    sub[k as usize - 1]
                } else {
                    -1
                }
            })
            .collect()
    }
}

struct Solution {}
pub fn run() {
    let nums = vec![1, 2, 3, 4, 3, 2, 5];
    tracing::info!("{:?}", Solution::results_array(nums, 3));
    let nums = vec![3, 2, 3, 2, 3, 2];
    tracing::info!("{:?}", Solution::results_array(nums, 2));
}
