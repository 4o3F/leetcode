impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut paper = String::with_capacity(s.len());
        let mut stack = Vec::<char>::with_capacity(s.len());
        let mut count: [u32; 26] = [0; 26];
        s.bytes().for_each(|b| count[(b - b'a') as usize] += 1);
        let mut idx: usize = 0;
        for c in s.chars() {
            stack.push(c);
            count[(c as u8 - b'a') as usize] -= 1;
            while idx < 26 && count[idx] == 0 {
                idx += 1;
            }
            while !stack.is_empty() && ((stack[stack.len() - 1] as u8) - b'a') as usize <= idx {
                paper.push(stack.pop().unwrap());
            }
        }
        paper
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!(
        "{}",
        Solution::robot_with_string("mmuqezwmomeplrtskz".to_string())
    )
}
