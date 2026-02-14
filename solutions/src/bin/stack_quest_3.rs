#[derive(Debug)]
enum Operation {
    Start,
    End,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "start" => Operation::Start,
            "end" => Operation::End,
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        // (program_id, last_running_timestamp, exclusive_time)
        let mut stack = Vec::<(usize, i32, i32)>::with_capacity(n as usize);
        for log in logs {
            let (program_id, remain) = log.split_once(":").unwrap();
            let (operation, timestamp) = remain.split_once(":").unwrap();
            let program_id = program_id.parse::<usize>().unwrap();
            let operation = Operation::from(operation);
            let timestamp = timestamp.parse::<i32>().unwrap();

            // dbg!(&stack);
            // dbg!(&result);
            // tracing::info!(
            //     "pid {} operation {:?} timestamp {}",
            //     program_id,
            //     operation,
            //     timestamp
            // );

            if stack.is_empty() {
                stack.push((program_id, timestamp, 0));
            } else {
                let previous = stack.pop().unwrap();
                match operation {
                    Operation::Start => {
                        // Stop the execution of previous program
                        stack.push((previous.0, timestamp, previous.2 + timestamp - previous.1));

                        // Start the execution of new program
                        stack.push((program_id, timestamp, 0));
                    }
                    Operation::End => {
                        assert_eq!(previous.0, program_id);
                        result[program_id] += timestamp - previous.1 + 1 + previous.2;
                        if !stack.is_empty() {
                            let previous = stack.pop().unwrap();
                            stack.push((previous.0, timestamp + 1, previous.2));
                        }
                    }
                }
            }
        }

        result
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{:?}",
        Solution::exclusive_time(
            2,
            [
                "0:start:0",
                "0:start:2",
                "0:end:5",
                "1:start:6",
                "1:end:6",
                "0:end:7"
            ]
            .iter()
            .map(|x| x.to_string())
            .collect()
        )
    );
    tracing::info!(
        "{:?}",
        Solution::exclusive_time(
            1,
            [
                "0:start:0",
                "0:start:2",
                "0:end:5",
                "0:start:6",
                "0:end:6",
                "0:end:7"
            ]
            .iter()
            .map(|x| x.to_string())
            .collect()
        )
    );
    tracing::info!(
        "{:?}",
        Solution::exclusive_time(
            2,
            ["0:start:0", "1:start:2", "1:end:5", "0:end:6"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
}
