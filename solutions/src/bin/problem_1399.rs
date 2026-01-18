use std::collections::HashMap;

impl Solution {
    fn digit_count(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }

    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::<i32, i32>::new();
        let mut max_size = 0;
        for i in 1..=n {
            let entry = map.entry(Self::digit_count(i)).or_insert(0);
            *entry += 1;
            max_size = max_size.max(*entry)
        }
        map.iter().filter(|&(_, &v)| v == max_size).count() as i32
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_largest_group(13))
}
