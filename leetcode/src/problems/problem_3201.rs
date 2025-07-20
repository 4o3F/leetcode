impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let all_even = nums.iter().filter(|&x| x % 2 == 0).count();
        let all_odd = nums.iter().filter(|&x| x % 2 == 1).count();
        let start_even = nums
            .iter()
            .fold((None, false), |(mut current_count, mut is_even), num| {
                match current_count {
                    None => {
                        if num % 2 == 0 {
                            is_even = true;
                            current_count = Some(1);
                        }
                    }
                    Some(count) => {
                        if num % 2 == if is_even { 1 } else { 0 } {
                            is_even = !is_even;
                            current_count = Some(count + 1);
                        }
                    }
                }
                (current_count, is_even)
            })
            .0
            .unwrap_or(0);
        let start_odd = nums
            .iter()
            .fold((None, false), |(mut current_count, mut is_odd), num| {
                match current_count {
                    None => {
                        if num % 2 == 1 {
                            is_odd = true;
                            current_count = Some(1);
                        }
                    }
                    Some(count) => {
                        if num % 2 == if is_odd { 0 } else { 1 } {
                            is_odd = !is_odd;
                            current_count = Some(count + 1);
                        }
                    }
                }
                (current_count, is_odd)
            })
            .0
            .unwrap_or(0);
        all_even.max(all_odd).max(start_even).max(start_odd) as i32
    }
}

struct Solution;

pub fn run() {
    tracing::info!("{}", Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]))
}
