// use std::collections::VecDeque;

impl Solution {
    // fn rle(input: String) -> String {
    //     let input = input.into_bytes();
    //     let mut result = VecDeque::<u8>::new();
    //     for ch in input {
    //         if result.is_empty() {
    //             result.push_back(ch);
    //         } else {
    //             if result.back().unwrap() == &ch {
    //                 result.push_back(ch);
    //             } else {
    //                 let mut same_count = '0' as u8;
    //                 let current_ch = result.back().unwrap().clone();
    //                 while !result.is_empty() && result.back().unwrap() == &current_ch {
    //                     result.pop_back();
    //                     same_count += 1;
    //                 }
    //                 // assert!(same_count < '0' as u8 + 10, "Count is more than 10");
    //                 result.push_back(same_count);
    //                 result.push_back(current_ch);
    //                 result.push_back(ch);
    //             }
    //         }
    //     }
    //     {
    //         let ch = result.back().unwrap().clone();
    //         let mut same_count = '0' as u8;
    //         while !result.is_empty() && result.back().unwrap() == &ch {
    //             result.pop_back();
    //             same_count += 1;
    //         }
    //         // assert!(same_count < '0' as u8 + 10, "Count is more than 10");
    //         result.push_back(same_count);
    //         result.push_back(ch);
    //     }
    //     // tracing::info!("Result {:?}", result);
    //     String::from_utf8(Vec::from(result)).unwrap()
    // }

    fn rle(input: String) -> String {
        let mut iter = input.chars().peekable();
        let mut result = String::new();
        while let Some(ch) = iter.next() {
            let mut count = 1;
            while let Some(next_char) = iter.peek() {
                if next_char == &ch {
                    count += 1;
                    iter.next();
                } else {
                    break;
                }
            }
            result.push_str(&count.to_string());
            result.push(ch);
        }
        result
    }

    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            "1".to_string()
        } else {
            let recur = Self::count_and_say(n - 1);
            Self::rle(recur.clone())
        }
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    tracing::info!("{}", Solution::count_and_say(1));
    tracing::info!("{}", Solution::count_and_say(4));
    tracing::info!("{}", Solution::count_and_say(7));
}
