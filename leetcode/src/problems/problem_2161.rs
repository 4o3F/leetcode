impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut lower = vec![];
        let mut upper = vec![];
        let mut equal_count = 0;
        for num in nums {
            match num.cmp(&pivot) {
                std::cmp::Ordering::Less => lower.push(num),
                std::cmp::Ordering::Equal => equal_count += 1,
                std::cmp::Ordering::Greater => upper.push(num),
            }
        }
        lower.extend(std::iter::repeat(pivot).take(equal_count));
        lower.extend(upper.into_iter());
        lower
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{:?}",
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
    );
}
