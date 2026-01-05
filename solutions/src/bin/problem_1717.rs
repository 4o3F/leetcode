impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        fn solve(s: String, c1: char, c2: char, x: i32, y: i32) -> i32 {
            let mut sum = 0;
            s.chars()
                .fold(Vec::new(), |mut stack, c| {
                    stack.push(c);
                    if stack.len() > 1
                        && stack[stack.len() - 2] == c1
                        && stack[stack.len() - 1] == c2
                    {
                        stack.pop();
                        stack.pop();
                        sum += x;
                    }
                    stack
                })
                .into_iter()
                .fold(Vec::new(), |mut stack, c| {
                    stack.push(c);
                    if stack.len() > 1
                        && stack[stack.len() - 2] == c2
                        && stack[stack.len() - 1] == c1
                    {
                        stack.pop();
                        stack.pop();
                        sum += y;
                    }
                    stack
                });
            sum
        }
        match x >= y {
            true => solve(s, 'a', 'b', x, y),
            false => solve(s, 'b', 'a', y, x),
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5)
    );
}
