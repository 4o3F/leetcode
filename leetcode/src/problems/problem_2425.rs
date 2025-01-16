impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();
        let mut num3 = Vec::new();
        for num in nums1 {
            if nums2_len % 2 == 0 {
                num3.push(0);
            } else {
                num3.push(num);
            }
        }

        for num in nums2 {
            if nums1_len % 2 == 0 {
                num3.push(0);
            } else {
                num3.push(num);
            }
        }
        num3.iter().fold(0, |acc, x| acc ^ x)
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0])
    );
}
