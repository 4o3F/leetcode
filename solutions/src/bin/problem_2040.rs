impl Solution {
    pub fn kth_smallest_product(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i64) -> i64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let (neg1, pos1, zero1) = Self::split_by_sign(&mut nums1);
        let (neg2, pos2, zero2) = Self::split_by_sign(&mut nums2);

        let neg_count = pos1.len() * neg2.len() + pos2.len() * neg1.len();
        let zero_count = zero1 * n2 + zero2 * n1 - zero1 * zero2;

        match k as usize {
            k if k <= neg_count => {
                -Self::kth_smallest_positive_product(pos1, neg2, pos2, neg1, neg_count - k + 1)
            }
            k if k <= neg_count + zero_count => 0,
            k => Self::kth_smallest_positive_product(
                neg1,
                neg2,
                pos1,
                pos2,
                k - neg_count - zero_count,
            ),
        }
    }

    // Splits a sorted array into:
    //   - reversed negatives [0..z1),
    //   - positives          [z2..),
    //   - and returns zero count as z2 - z1
    fn split_by_sign(nums: &mut [i32]) -> (&[i32], &[i32], usize) {
        let mut z1 = nums.len(); // index of first non-negative
        for i in 0..nums.len() {
            if nums[i] < 0 {
                nums[i] = -nums[i];
            } else {
                z1 = i;
                break;
            }
        }

        let z2 = z1
            + nums[z1..]
                .binary_search_by(|&x| {
                    if x <= 0 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
                .unwrap_or_else(|i| i);

        nums[..z1].reverse();
        (&nums[..z1], &nums[z2..], z2 - z1)
    }

    // Counts number of (i,j) such that nums1[i] * nums2[j] <= val
    fn count_products_leq(nums1: &[i32], nums2: &[i32], val: i64) -> usize {
        if nums1.is_empty() || nums2.is_empty() {
            return 0;
        }

        let mut count = 0;
        let mut j = nums2.len();
        for &x in nums1 {
            while j > 0 && (x as i64) * (nums2[j - 1] as i64) > val {
                j -= 1;
            }
            count += j;
        }
        count
    }

    // Binary search for the k-th smallest product in a1×a2 ∪ b1×b2
    fn kth_smallest_positive_product(
        a1: &[i32],
        a2: &[i32],
        b1: &[i32],
        b2: &[i32],
        k: usize,
    ) -> i64 {
        let mut r = 0;
        if let (Some(&max1), Some(&max2)) = (a1.last(), a2.last()) {
            r = (max1 as i64) * (max2 as i64);
        }
        if let (Some(&max1), Some(&max2)) = (b1.last(), b2.last()) {
            r = r.max((max1 as i64) * (max2 as i64));
        }

        let mut l = 0;
        while l < r {
            let m = l + (r - l) / 2;
            let count = Self::count_products_leq(a1, a2, m) + Self::count_products_leq(b1, b2, m);
            if count >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::kth_smallest_product(vec![2, 5], vec![3, 4], 2)
    )
}
