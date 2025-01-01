impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let (mut i1, mut i12, mut i123) = (0, (0, k), [0, k, 2 * k]);
        let mut s1 = nums[0..k].iter().sum::<i32>();
        let mut s2 = nums[k..2 * k].iter().sum::<i32>();
        let mut s3 = nums[2 * k..3 * k].iter().sum::<i32>();
        let (mut m1, mut m12, mut m123) = (s1, s1 + s2, s1 + s2 + s3);
        for i in 3 * k..nums.len() {
            s1 += nums[i - 2 * k] - nums[i - 3 * k];
            s2 += nums[i - k] - nums[i - 2 * k];
            s3 += nums[i] - nums[i - k];
            if s1 > m1 {
                m1 = s1;
                i1 = i - 3 * k + 1
            }
            if m1 + s2 > m12 {
                m12 = m1 + s2;
                i12 = (i1, i - 2 * k + 1)
            }
            if m12 + s3 > m123 {
                m123 = m12 + s3;
                i123 = [i12.0, i12.1, i - k + 1]
            }
        }
        i123.iter().map(|&x| x as i32).collect()
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2)
    );
}
