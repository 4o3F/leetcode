use std::iter::successors;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let modulo = 10_i32.pow(9) + 7;
        let powers = successors(Some(1), |prev| Some((prev * 2) % modulo))
            .take(nums.len() + 1)
            .collect::<Vec<_>>();
        nums.iter()
            .enumerate()
            .scan(nums.len(), |i_end, (i, &n)| {
                while *i_end > i && n + nums[*i_end - 1] > target {
                    *i_end -= 1;
                }
                (*i_end > i).then(|| powers[*i_end - i - 1])
            })
            .reduce(|sum, s| (sum + s) % modulo)
            .unwrap_or(0)
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::num_subseq(vec![3, 5, 6, 7], 9));
}
