impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut odd_total_sum = 0;
        let mut even_total_sum = 0;
        for (idx, &num) in nums.iter().enumerate() {
            let num = num as i64;
            if idx % 2 == 0 {
                even_total_sum += num;
            } else {
                odd_total_sum += num;
            }
        }
        let mut odd_before_sum = 0;
        let mut even_before_sum = 0;
        let mut result = 0;
        for (idx, &num) in nums.iter().enumerate() {
            let num = num as i64;

            let (odd_after_sum, even_after_sum) = if idx % 2 == 0 {
                (odd_total_sum - odd_before_sum, even_total_sum - even_before_sum - num)
            } else {
                (odd_total_sum - odd_before_sum - num, even_total_sum - even_before_sum)
            };
            if odd_before_sum + even_after_sum == even_before_sum + odd_after_sum {
                result += 1;
            }
            if idx % 2 == 0 {
                even_before_sum += num;
            } else {
                odd_before_sum += num;
            }
        }
        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{}", Solution::ways_to_make_fair(vec![2, 1, 6, 4]));
}
