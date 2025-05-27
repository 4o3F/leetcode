use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut appeared = HashMap::<String, u32>::new();
        let mut length = 0;
        let mut center_reversed_count = 0;
        for word in words {
            let reversed = word.chars().rev().collect::<String>();

            if appeared.contains_key(&reversed) && *appeared.get(&reversed).unwrap() > 0 {
                *appeared.get_mut(&reversed).unwrap() -= 1;
                if reversed == word {
                    center_reversed_count -= 1;
                    length += 2;
                    if center_reversed_count > 0 {
                        length += 2;
                    }
                } else {
                    length += 4;
                }
            } else {
                if reversed == word {
                    if center_reversed_count == 0 {
                        length += 2;
                    }
                    center_reversed_count += 1;
                }
                let entry = appeared.entry(word).or_insert(0);
                *entry += 1;
            }
            // tracing::info!("{:?} {} {}", appeared, length, center_reversed_count);
        }
        length
    }
}

struct Solution;

pub fn run() {
    let words = vec![
        "qo", "fo", "fq", "qf", "fo", "ff", "qq", "qf", "of", "of", "oo", "of", "of", "qf", "qf",
        "of",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>();

    // words.sort();
    tracing::info!("words {:?}", words);
    tracing::info!("{}", Solution::longest_palindrome(words))
}
