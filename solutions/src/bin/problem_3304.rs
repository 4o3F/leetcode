impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut word = Vec::from(['a']);

        fn next_char(c: char) -> char {
            match c {
                'a'..='y' => ((c as u8) + 1) as char,
                'z' => 'a',
                'A'..='Y' => ((c as u8) + 1) as char,
                'Z' => 'A',
                _ => unreachable!(),
            }
        }
        while word.len() < k as usize {
            let mut new = word.iter().map(|x| next_char(*x)).collect::<Vec<_>>();
            word.append(&mut new);
        }
        // tracing::info!("{:?}", word);
        word[k as usize]
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::kth_character(5))
}
