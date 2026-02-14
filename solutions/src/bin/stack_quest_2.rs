impl Solution {
    fn is_op(s: &str) -> bool {
        matches!(s, "+" | "-" | "*" | "/")
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::with_capacity(tokens.len());
        for token in tokens {
            // dbg!(&stack);
            if !Self::is_op(&token) {
                stack.push(token.parse::<i32>().unwrap());
            } else {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                // tracing::info!("op {} num1 {} num2 {}", token, num1, num2);
                let new_num = match token.as_str() {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => unreachable!(),
                };
                stack.push(new_num);
            }
        }

        assert_eq!(stack.len(), 1);

        stack[0]
    }
}

struct Solution;
use utils::logger::init_logger;
fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::eval_rpn(
            [
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]
            .iter()
            .map(|x| x.to_string())
            .collect()
        )
    )
}
