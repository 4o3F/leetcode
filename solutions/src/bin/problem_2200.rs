use std::collections::HashSet;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut key_idxs = Vec::<usize>::new();
        for (idx, &num) in nums.iter().enumerate() {
            if num == key {
                key_idxs.push(idx);
            }
        }

        // tracing::info!("{:?}", key_idxs);

        let mut result = HashSet::<i32>::new();
        for key_idx in key_idxs {
            let key_idx = key_idx as i32;
            for idx in (key_idx - k).max(0)..=(key_idx + k).min((nums.len() - 1) as i32) {
                result.insert(idx);
            }
        }
        let mut result = result.into_iter().collect::<Vec<_>>();
        result.sort_unstable();
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::find_k_distant_indices(
            vec![
                734, 228, 636, 204, 552, 732, 686, 461, 973, 874, 90, 537, 939, 986, 855, 387, 344,
                939, 552, 389, 116, 93, 545, 805, 572, 306, 157, 899, 276, 479, 337, 219, 936, 416,
                457, 612, 795, 221, 51, 363, 667, 112, 686, 21, 416, 264, 942, 2, 127, 47, 151,
                277, 603, 842, 586, 630, 508, 147, 866, 434, 973, 216, 656, 413, 504, 360, 990,
                228, 22, 368, 660, 945, 99, 685, 28, 725, 673, 545, 918, 733, 158, 254, 207, 742,
                705, 432, 771, 578, 549, 228, 766, 998, 782, 757, 561, 444, 426, 625, 706, 946
            ],
            939,
            34
        )
    );
}
