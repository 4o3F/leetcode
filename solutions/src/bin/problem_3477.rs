impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut taken = vec![false; baskets.len()];

        let mut result = 0;
        let mut placed = 0;
        for fruit in fruits {
            if placed == baskets.len() {
                result += 1;
                continue;
            }
            let mut flag = false;
            for idx in 0..baskets.len() {
                if taken[idx] {
                    continue;
                } else if baskets[idx] >= fruit {
                    taken[idx] = true;
                    placed += 1;
                    flag = true;
                    break;
                }
            }
            if !flag {
                result += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4])
    )
}
