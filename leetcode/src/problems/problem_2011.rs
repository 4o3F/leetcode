impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(
            0,
            |acc, op| {
                if op.contains("+") {
                    acc + 1
                } else {
                    acc - 1
                }
            },
        )
    }
}

struct Solution;

pub fn run() {
    tracing::info!(
        "{}",
        Solution::final_value_after_operations(
            vec!["--X", "X++", "X++"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    )
}
