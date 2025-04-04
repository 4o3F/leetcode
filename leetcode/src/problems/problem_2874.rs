impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let (mut max_element, mut max_diff) = (0, 0);
        nums.iter()
            .map(|&num| {
                let num = num as i64;
                let triplet = max_diff * num;
                max_diff = max_diff.max(max_element - num);
                max_element = max_element.max(num);
                triplet
            })
            .max()
            .unwrap()
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
}
