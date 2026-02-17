impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count = [0; 26];
        let mut appeared = [false; 26];
        let mut stack = Vec::<char>::with_capacity(s.len());
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        // dbg!(count);
        for curr_char in s.chars() {
            // tracing::info!(
            //     "curr_char {} stack {:?} count {:?}",
            //     curr_char,
            //     stack,
            //     count
            // );
            while let Some(&last_char) = stack.last()
                && curr_char < last_char
                && count[last_char as usize - 'a' as usize] > 0
                && !appeared[curr_char as usize - 'a' as usize]
            {
                stack.pop();
                appeared[last_char as usize - 'a' as usize] = false;
            }
            if !appeared[curr_char as usize - 'a' as usize] {
                stack.push(curr_char);
                appeared[curr_char as usize - 'a' as usize] = true;
            }
            count[curr_char as usize - 'a' as usize] -= 1;
        }
        stack.into_iter().collect()
    }
}

struct Solution;

use utils::logger::init_logger;

fn main() {
    init_logger();
    tracing::info!(
        "{}",
        Solution::remove_duplicate_letters("bbcaac".to_string())
    );
    // tracing::info!(
    //     "{}",
    //     Solution::remove_duplicate_letters("cbacdcbc".to_string())
    // );
    // tracing::info!(
    //     "{}",
    //     Solution::remove_duplicate_letters("bcabc".to_string())
    // );
}
