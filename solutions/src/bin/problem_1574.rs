impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let (mut low, mut high) = (0, n - 1);
        while high > 0 && arr[high - 1] <= arr[high] {
            high -= 1;
        }
        let mut result = high;
        while low < high && high <= n && (low < 1 || arr[low] >= arr[low - 1]) {
            if high == n || arr[low] <= arr[high] {
                result = result.min(high - low - 1);
                low += 1;
            } else {
                high += 1;
            }
        }
        result as i32
    }
}

struct Solution {}

fn main() {
    use utils::prelude::*;
    init_logger();
    let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
    tracing::info!("{}", Solution::find_length_of_shortest_subarray(arr));
}
