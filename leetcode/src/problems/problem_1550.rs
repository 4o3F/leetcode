impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        for i in 0..=arr.len() - 3 {
            if arr[i] % 2 == 1 && arr[i + 1] % 2 == 1 && arr[i + 2] % 2 == 1 {
                return true;
            }
        }
        false
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12])
    )
}
