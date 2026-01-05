impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let length = arr.len();

        let mut count = 0;
        for i in 0..length {
            for j in i + 1..length {
                if (arr[j] - arr[i]).abs() > a {
                    continue;
                }
                for k in j + 1..length {
                    if (arr[k] - arr[j]).abs() > b || (arr[k] - arr[i]).abs() > c {
                        continue;
                    }
                    // tracing::info!("{} {} {}", arr[i], arr[j], arr[k]);
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
    );
    tracing::info!(
        "{}",
        Solution::count_good_triplets(vec![7, 3, 7, 3, 12, 1, 12, 2, 3], 5, 8, 1)
    );
}
