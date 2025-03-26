impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = grid.into_iter().flatten().collect::<Vec<_>>();
        nums.sort_unstable();
        let medium = nums[nums.len() / 2];
        let mut result = 0;
        for num in nums {
            if (num - medium).abs() % x == 0 {
                result += (num - medium).abs() / x;
            } else {
                return -1;
            }
        }
        result
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2)
    );
}
