impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut low, mut high) = (1, *nums.iter().max().unwrap());
        while low < high {
            let mid = (low + high) / 2;
            let ops = nums
                .iter()
                .map(|&num| ((num + mid - 1) / mid) - 1)
                .sum::<i32>();
            // tracing::debug!("low: {}, high: {}, mid: {}, ops: {}", low, high, mid, ops);
            if ops <= max_operations {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }
}

struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::minimum_size(vec![9], 2));
}
