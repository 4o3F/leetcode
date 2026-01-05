impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn is_valid_partition(s: &str, target: i32) -> bool {
            fn dfs(s: &str, sum: i32, target: i32) -> bool {
                if s.is_empty() {
                    return sum == target;
                }
                (1..=s.len()).any(|i| {
                    s[..i]
                        .parse::<i32>()
                        .is_ok_and(|num| dfs(&s[i..], sum + num, target))
                })
            }
            dfs(s, 0, target)
        }

        (1..=n)
            .filter(|&i| is_valid_partition(&(i * i).to_string(), i))
            .map(|i| i * i)
            .sum()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::punishment_number(10));
}
