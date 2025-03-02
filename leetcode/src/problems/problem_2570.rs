impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let (mut idx1, mut idx2) = (0, 0);
        while idx1 < nums1.len() && idx2 < nums2.len() {
            if nums1[idx1][0] < nums2[idx2][0] {
                result.push(nums1[idx1].clone());
                idx1 += 1;
            } else if nums1[idx1][0] > nums2[idx2][0] {
                result.push(nums2[idx2].clone());
                idx2 += 1;
            } else {
                result.push(vec![nums1[idx1][0], nums1[idx1][1] + nums2[idx2][1]]);
                idx1 += 1;
                idx2 += 1;
            }
        }
        while idx1 < nums1.len() {
            result.push(nums1[idx1].clone());
            idx1 += 1;
        }
        while idx2 < nums2.len() {
            result.push(nums2[idx2].clone());
            idx2 += 1;
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::merge_arrays(
            [[1, 2], [2, 3], [4, 5]]
                .into_iter()
                .map(|x| x.to_vec())
                .collect::<Vec<_>>(),
            [[1, 4], [3, 2], [4, 1]]
                .into_iter()
                .map(|x| x.to_vec())
                .collect::<Vec<_>>()
        )
    )
}
