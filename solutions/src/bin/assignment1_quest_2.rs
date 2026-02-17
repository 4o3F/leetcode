enum Status {
    Initial,
    Up,
    Down,
}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut curr_status = Status::Initial;

        let mut iter = arr.iter().peekable();
        while let Some(curr_num) = iter.next()
            && let Some(&next_num) = iter.peek()
        {
            // tracing::info!("{} {}", curr_num, next_num)
            match curr_status {
                Status::Initial => {
                    if next_num > curr_num {
                        curr_status = Status::Up
                    } else {
                        return false;
                    }
                }
                Status::Up => {
                    if next_num < curr_num {
                        curr_status = Status::Down;
                    } else if next_num == curr_num {
                        return false;
                    }
                }
                Status::Down => {
                    if next_num >= curr_num {
                        return false;
                    }
                }
            }
        }
        if !matches!(curr_status, Status::Down) {
            return false;
        }
        true
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    );
}
