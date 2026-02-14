enum Operation {
    Push,
    Pop,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Pop => write!(f, "Pop"),
            Operation::Push => write!(f, "Push"),
        }
    }
}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = Vec::<Operation>::new();

        let mut stack = Vec::<i32>::new();
        let mut current_layer = 0;
        for new_num in 1..=n {
            if stack == target {
                break;
            } else {
                stack.push(new_num);
                result.push(Operation::Push);
                if stack[current_layer] != target[current_layer] {
                    stack.pop();
                    result.push(Operation::Pop);
                } else {
                    current_layer += 1;
                }
            }
        }

        result.iter().map(|x| x.to_string()).collect()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!("{:?}", Solution::build_array(vec![1, 2], 4));
}
